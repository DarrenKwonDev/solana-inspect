use std::env;

use anyhow::Result;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init();
    dotenv::dotenv().ok();

    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    println!("rpc_url : {}", rpc_url);

    // create rpc
    let rpc = RpcClient::new(rpc_url);

    // 1. 현재 slot 조회
    let slot = rpc.get_slot()?;
    println!("Current slot: {}", slot);

    // 2. Block 가져오기
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(false),
        commitment: None,
        max_supported_transaction_version: Some(0),
    };

    let block = rpc.get_block_with_config(slot, config)?;
    println!("Block time: {}", block.block_time.unwrap_or(0));

    // 3. 첫 10개 TX 출력
    if let Some(txs) = &block.transactions {
        println!("TX count: {}", txs.len());

        println!("\nFirst 10 transactions:");
        for (idx, tx_meta) in txs.iter().take(10).enumerate() {
            // tx_meta의 transaction 필드에서 signature 정보 가져오기
            let sig = match &tx_meta.transaction {
                solana_transaction_status::EncodedTransaction::Json(tx) => {
                    if !tx.signatures.is_empty() {
                        &tx.signatures[0]
                    } else {
                        "unknown"
                    }
                }
                _ => "unknown",
            };

            println!("[{}] {}", idx, sig);
        }
    } else {
        println!("No transactions in block");
    }

    Ok(())
}
