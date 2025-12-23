use std::{collections::HashMap, env};

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const JUP_API_TOKEN: &str = "https://api.jup.ag/tokens/v2";

pub enum TokenTagType {
  Lst,
  Vrfd,
}

impl TokenTagType {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Lst => "lst",
      Self::Vrfd => "verified",
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenMetadata {
  #[serde(default)]
  #[serde(rename = "id")]
  pub mint_addr: String,

  #[serde(default)]
  pub symbol: String,

  #[serde(default)]
  pub name: String,

  #[serde(default)]
  pub decimals: u8,
}

pub async fn fetch_token_all(query: TokenTagType) -> Result<HashMap<String, TokenMetadata>> {
  let client = reqwest::Client::new();
  let api_key = env::var("JUPITER_API_KEY")?;
  let resp: Vec<TokenMetadata> = client
    .get(format!("{}/tag?query={}", JUP_API_TOKEN, query.as_str()))
    .header("x-api-key", api_key)
    .send()
    .await?
    .json()
    .await?;

  let mut token_map: HashMap<String, TokenMetadata> = HashMap::new();
  for token in resp {
    token_map.insert(token.mint_addr.clone(), token);
  }

  Ok(token_map)
}
