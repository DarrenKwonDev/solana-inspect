use std::env;

use anyhow::Result;
use solana_client::nonblocking::pubsub_client::PubsubClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let wss_url = env::var("SOLANA_WSS_URL").expect("fail to parse SOLANA_WSS_URL");
    println!("{}", wss_url);

    let pubsub = PubsubClient::new(&wss_url).await?;

    println!("{:?}", pubsub);

    Ok(())
}
