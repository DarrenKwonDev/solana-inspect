use std::env;

use anyhow::Result;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use solana_sdk::commitment_config::CommitmentConfig;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let wss_url = env::var("SOLANA_WSS_URL").expect("fail to parse SOLANA_WSS_URL");
    println!("{}", wss_url);

    let pubsub = PubsubClient::new(&wss_url).await?;

    // 모든 tx를 분석한다는 목표라면
    // slot_subscribe + get_block 조합 사용하는 것이 좋음
    // get_slot이 반환하는 slot은 최신이 아닐 수 있다. RPC 지연이 100ms를 초과하면 슬롯을 놓친다.
    /*
        pub async fn monitor_all_txs_subscribe(rpc: &RpcClient) {
            let mut subscription = rpc.slot_subscribe()?;

            while let Some(slot_update) = subscription.next().await {
                let slot = slot_update.slot;

                match rpc.get_block(slot) {
                    Ok(block) => {
                        process_all_txs(&block);  // 모든 TX 처리
                    }
                    Err(_) => {
                        // 스킵된 슬롯 (네트워크 불안정)
                        // → 다음 신호 기다림
                    }
                }
            }
        }
    */

    Ok(())
}
