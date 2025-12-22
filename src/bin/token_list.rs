use anyhow::Result;
use solana_inspect::{
  CACHE_TOKEN_FILE_NAME,
  fetcher::{
    cache::{Cache, get_cache_dir},
    token::TokenMetadataMap,
  },
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
  let cache_dir = get_cache_dir()?;
  println!("{:?}", cache_dir);

  let token_cache = Arc::new(Cache::<TokenMetadataMap>::new(
    cache_dir.join(CACHE_TOKEN_FILE_NAME),
  ));

  let _ = token_cache.load();

  // fetch jup verfied

  // save into file

  Ok(())
}
