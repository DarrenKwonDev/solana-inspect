use anyhow::Result;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_inspect::dex_filter;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{TransactionDetails, UiConfirmedBlock, UiTransactionEncoding};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // --------------------------------
    // setup
    // --------------------------------
    dotenv::dotenv().ok();

    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    // println!("rpc_url : {}", rpc_url);
    let rpc = RpcClient::new(rpc_url);

    let offset = 1;

    // --------------------------------
    // slot and block
    // --------------------------------
    let slot = rpc.get_slot()?;
    println!("Current slot: {}", slot);
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

    for offset in 0..=offset {
        let block = rpc.get_block_with_config(slot - offset, config)?;
        println!("Block time: {}", block.block_time.unwrap_or(0));
        parse_tx_in_block(&block);
    }

    Ok(())
}

fn parse_tx_in_block(block: &UiConfirmedBlock) {
    // --------------------------------
    // counters
    // --------------------------------
    let mut parsed_message_cnt = 0;
    let mut raw_message_cnt = 0;

    let mut compiled_instr_cnt = 0;
    let mut parsed_instr_cnt = 0;

    let mut full_instr_cnt = 0;
    let mut partial_instr_cnt = 0;

    let mut dex_count = 0;

    // --------------------------------
    // parse transactions
    // --------------------------------
    if let Some(txs) = &block.transactions {
        println!("Total TXs in block: {}", txs.len());

        // msg parsing
        for tx_meta in txs.iter() {
            match &tx_meta.transaction {
                solana_transaction_status::EncodedTransaction::Json(tx) => {
                    /*
                        // tx는 signature와 message로 구성되어 있다.
                        pub struct UiTransaction {
                            pub signatures: Vec<String>,
                            pub message: UiMessage,
                        }

                        // encoding이 Json일 경우 Parsed, Raw 둘 다 로직을 작성해야 함.
                        // encoding이 JsonParsed일 경우엔 Parsed 만 존재하지만 파싱이 실패한 경우 데이터가 좀 빠져있을 수도 있음.
                        // 정말로 다 보고 싶으면 애초에 Binary 형태로 요청해서 받아야 함
                        pub enum UiMessage {
                            Parsed(UiParsedMessage), -> Json 요청, JsonParsed의 전부
                            Raw(UiRawMessage), -> Json 요청시 fallback으로 올 경우
                        }

                        // JsonParsed의 경우 모두 ParsedMessage로 옮
                        pub struct UiParsedMessage {
                            pub account_keys: Vec<ParsedAccount>,
                            pub recent_blockhash: String,
                            pub instructions: Vec<UiInstruction>,
                            #[serde(default, skip_serializing_if = "Option::is_none")]
                            pub address_table_lookups: Option<Vec<UiAddressTableLookup>>,
                        }

                        pub enum UiInstruction {
                            Compiled(UiCompiledInstruction),
                            Parsed(UiParsedInstruction), // JsonParsed인 경우
                        }

                        // parsed instruction이라고 하더라도 두 경우가 존재함
                        pub enum UiParsedInstruction {
                            Parsed(ParsedInstruction),
                            PartiallyDecoded(UiPartiallyDecodedInstruction),
                        }

                        pub struct ParsedInstruction {
                            pub program: String,
                            pub program_id: String,
                            pub parsed: Value,
                            pub stack_height: Option<u32>,
                        }

                        pub struct UiPartiallyDecodedInstruction {
                            pub program_id: String,
                            pub accounts: Vec<String>,
                            pub data: String,
                            pub stack_height: Option<u32>,
                        }

                    */

                    // 1. message가 Parsed냐 Raw냐
                    match &tx.message {
                        solana_transaction_status::UiMessage::Parsed(ui_parsed_message) => {
                            parsed_message_cnt += 1;

                            for parsed_instr in ui_parsed_message.instructions.iter() {
                                match &parsed_instr {
                                    solana_transaction_status::UiInstruction::Parsed(
                                        ui_parsed_instruction,
                                    ) => {
                                        parsed_instr_cnt += 1;

                                        let program_id_str = match &ui_parsed_instruction {
                                            solana_transaction_status::UiParsedInstruction::Parsed(instr) => {
                                                full_instr_cnt+=1;
                                                &instr.program_id
                                            },
                                            solana_transaction_status::UiParsedInstruction::PartiallyDecoded(partial_instr)=>{
                                                partial_instr_cnt+=1;
                                                &partial_instr.program_id
                                            },
                                        };

                                        if let Ok(pubkey) = program_id_str.parse::<Pubkey>() {
                                            if let Some(_) = dex_filter::is_dex_program(&pubkey) {
                                                dex_count += 1;
                                            }

                                            if dex_count < 50 {
                                                println!("Program: {}", program_id_str);
                                            }
                                        }
                                    }
                                    solana_transaction_status::UiInstruction::Compiled(_) => {
                                        compiled_instr_cnt += 1
                                    }
                                }
                            }
                        }
                        solana_transaction_status::UiMessage::Raw(_) => {
                            raw_message_cnt += 1;
                        }
                    }
                }
                solana_transaction_status::EncodedTransaction::Binary(_, _) => {}
                _ => {}
            }
        }

        println!("====== parsing count ======");
        // println!("encoding: {:?}", config.encoding);
        println!(
            "parsed msg : {} | raw msg: {}",
            parsed_message_cnt, raw_message_cnt
        );
        println!(
            "compiled instr: {} | parsed instr: {}",
            compiled_instr_cnt, parsed_instr_cnt
        );
        println!(
            "parsed instr {} -> full instr : {} | partial instr : {}",
            parsed_instr_cnt, full_instr_cnt, partial_instr_cnt
        );

        println!("\nDEX TXs in block [{}]: {}", block.blockhash, dex_count);
        println!("=============================");
    }
}
