use anyhow::Result;
use solana_inspect::{
  CACHE_TOKEN_FILE_NAME,
  fetcher::{
    cache::{Cache, get_cache_dir},
    jupyter_api::{TokenMetadata, TokenTagType, fetch_token_all},
  },
};
use solana_inspect::{GREEN, RESET};
use std::{collections::HashMap, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
  // ----------------------
  // setup
  // ----------------------
  dotenv::dotenv().ok();

  // ----------------------
  // api cache layer setup
  // ----------------------
  let cache_dir = get_cache_dir()?;
  println!("{:?}", cache_dir);

  let token_cache = Arc::new(Cache::<HashMap<String, TokenMetadata>>::new(
    cache_dir.join(CACHE_TOKEN_FILE_NAME),
  ));

  let _ = token_cache.load();

  // ---------------------------------
  // logics
  // ---------------------------------
  let token_map = token_cache
    .get("token:all", || fetch_token_all(TokenTagType::Vrfd))
    .await?;

  for (_, token) in token_map.iter() {
    println!(
      "{GREEN}{:<12}({:<4}){RESET} [{}]",
      token.symbol, token.decimals, token.mint_addr
    )
  }

  // save into file
  token_cache.persist()?;

  Ok(())
}
