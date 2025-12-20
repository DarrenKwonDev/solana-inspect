use anyhow::Result;
use futures_util::StreamExt;
use solana_client::rpc_config::RpcBlockConfig;
use solana_inspect::client;
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
    let rpc = client::rpc();
    let pubsub = client::pubsub();
    const SLOT_OFFSET: u64 = 30; // you can't see block inner data as soon as receive slot update.

    // ---------------------------------
    // parse previous block
    // 아래 코드는 slot이 400ms 단위로 업데이트 되며, 일반적으로 single core에서 핸들링하기에 충분하다.
    // ---------------------------------
    // 모든 tx를 분석한다는 목표라면
    // slot_subscribe + get_block 조합 사용하는 것이 좋음
    // get_slot이 반환하는 slot은 최신이 아닐 수 있다. RPC 지연이 크면 이전 슬롯을 줄수도 있다.
    let (mut slot_receiver, _unsub) = pubsub.slot_subscribe().await?;

    while let Some(slot_update) = slot_receiver.next().await {
        let slot = slot_update.slot;
        println!("slot update {}", slot);

        match get_block_by(slot - SLOT_OFFSET).await {
            Ok(block) => {
                println!("blockhash {}", block.blockhash)
            }
            Err(e) => {
                eprintln!("error {}", e)
            }
        }
    }

    Ok(())
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
