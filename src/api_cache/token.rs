use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TokenMetadata {
  pub mint: String,
  pub symbol: String,
  pub decimal: u8,
  pub name: String,
  // 필요하면 추가
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TokenMetadataMap {
  pub tokens: std::collections::HashMap<String, TokenMetadata>,
}
