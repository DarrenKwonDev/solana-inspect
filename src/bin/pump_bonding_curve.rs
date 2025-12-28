use anyhow::Result;
use solana_inspect::client;

/*
Create 곧바로 감지. (timestamp 찍고)
감지 후 buy, sell (timestamp 찍고) 마다 bonding curve 가격 계산. 대부분 스나이핑 3분 내로 끝나므로 3분 정도만 데이터 수집.
*/

#[tokio::main]
async fn main() -> Result<()> {
  // ---------------------------------
  // setup
  // ---------------------------------
  dotenv::dotenv().ok();

  // rpc, pubsub
  client::init_clients().await?;

  Ok(())
}
