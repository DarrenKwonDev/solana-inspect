use anyhow::Result;
use solana_client::nonblocking::{
    pubsub_client::PubsubClient, rpc_client::RpcClient as NonBlockingRpcClient,
};
use std::{
    env,
    sync::{Arc, OnceLock},
};

static RPC_CLIENT: OnceLock<Arc<NonBlockingRpcClient>> = OnceLock::new();
static PUBSUB_CLIENT: OnceLock<Arc<PubsubClient>> = OnceLock::new();

async fn init_rpc_client() -> Result<Arc<NonBlockingRpcClient>> {
    let rpc_url = env::var("SOLANA_RPC_URL").expect("fail to parse SOLANA_RPC_URL");
    println!("Initializing RpcClient with URL: {}", rpc_url);
    Ok(Arc::new(NonBlockingRpcClient::new(rpc_url)))
}

async fn init_pubsub_client() -> Result<Arc<PubsubClient>> {
    let wss_url = env::var("SOLANA_WSS_URL").expect("fail to parse SOLANA_WSS_URL");
    println!("Initializing PubsubClient with URL: {}", wss_url);
    let pubsub = PubsubClient::new(&wss_url).await?;
    Ok(Arc::new(pubsub))
}

pub fn rpc() -> Arc<NonBlockingRpcClient> {
    Arc::clone(
        RPC_CLIENT
            .get()
            .expect("RPC client not initialized. Call init_clients() first."),
    )
}

pub fn pubsub() -> Arc<PubsubClient> {
    Arc::clone(
        PUBSUB_CLIENT
            .get()
            .expect("Pubsub client not initialized. Call init_clients() first."),
    )
}

pub async fn init_clients() -> Result<()> {
    let rpc = init_rpc_client().await?;
    RPC_CLIENT.set(rpc).ok();

    let pubsub = init_pubsub_client().await?;
    PUBSUB_CLIENT.set(pubsub).ok();

    Ok(())
}
