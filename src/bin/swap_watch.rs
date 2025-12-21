use std::str::FromStr;

use anyhow::Result;
use futures_util::StreamExt;
use solana_client::rpc_config::RpcBlockConfig;
use solana_inspect::{
  client,
  dex_filter::{self},
};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{TransactionDetails, UiConfirmedBlock, UiTransactionEncoding};

#[tokio::main]
async fn main() -> Result<()> {
  // ---------------------------------
  // setup
  // ---------------------------------
  dotenv::dotenv().ok();
  client::init_clients().await?;

  // ---------------------------------
  // vars
  // ---------------------------------
  let _rpc = client::rpc();
  let pubsub = client::pubsub();
  const SLOT_OFFSET: u64 = 35; // you can't see block inner data as soon as receive slot update.

  // ---------------------------------
  // parse previous block
  // 모든 tx를 분석한다는 목표라면 slot_subscribe + get_block 조합 사용하는 것이 나은 듯.
  // ---------------------------------
  let (mut slot_receiver, _unsub) = pubsub.slot_subscribe().await?;

  while let Some(slot_update) = slot_receiver.next().await {
    let slot = slot_update.slot;
    println!("slot update {}", slot);

    match get_block_by(slot - SLOT_OFFSET).await {
      Ok(block) => {
        parse_tx_in_block(&block);
      }
      Err(e) => {
        eprintln!("error {}", e)
      }
    }
  }

  Ok(())
}

fn parse_tx_in_block(block: &UiConfirmedBlock) {
  if let Some(txs) = &block.transactions {
    for tx_meta in txs.iter() {
      match &tx_meta.transaction {
        solana_transaction_status::EncodedTransaction::Json(tx) => match &tx.message {
          solana_transaction_status::UiMessage::Parsed(ui_parsed_message) => {
            for instr in ui_parsed_message.instructions.iter() {
              match instr {
                solana_transaction_status::UiInstruction::Parsed(ui_parsed_instruction) => {
                  match &ui_parsed_instruction {
                    solana_transaction_status::UiParsedInstruction::Parsed(parsed_instruction) => {
                      if let Ok(pubkey) = Pubkey::from_str(&parsed_instruction.program_id) {
                        if dex_filter::is_swap(&pubkey).is_some() {
                          dbg!(parsed_instruction);
                        }
                      }
                    }
                    solana_transaction_status::UiParsedInstruction::PartiallyDecoded(
                      ui_partially_decoded_instruction,
                    ) => {
                      if let Ok(pubkey) =
                        Pubkey::from_str(&ui_partially_decoded_instruction.program_id)
                      {
                        if dex_filter::is_swap(&pubkey).is_some() {
                          dbg!(ui_partially_decoded_instruction);
                        }
                      }
                    }
                  }
                }
                _ => {}
              }
            }
          }
          _ => {}
        },
        _ => {}
      }

      // panic!("intentional panic");
    }
  }
}

async fn get_block_by(slot: u64) -> Result<UiConfirmedBlock> {
  let config = RpcBlockConfig {
    // encoding에 따라 solana_transaction_status::EncodedTransaction 가 달라짐
    // UiTransactionEncoding::JsonParsed → EncodedTransaction::Json
    // UiTransactionEncoding::Json        → EncodedTransaction::Json (또는 Raw fallback)
    // UiTransactionEncoding::Base64      → EncodedTransaction::Binary(String, Base64)
    // UiTransactionEncoding::Base58      → EncodedTransaction::LegacyBinary(String)
    // UiTransactionEncoding::Binary      → EncodedTransaction::Binary(String, Base64)
    encoding: Some(UiTransactionEncoding::JsonParsed), // Binary, Base64, Base58, Json, Jsonparsed
    transaction_details: Some(TransactionDetails::Full), // Full, Signatures, None, Accounts,
    rewards: Some(false),
    commitment: None,
    max_supported_transaction_version: Some(0),
  };

  let block = client::rpc().get_block_with_config(slot, config).await?;
  // println!("Block time: {}", block.block_time.unwrap_or(0));

  Ok(block)
}
