use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Result, anyhow};
use dashmap::DashMap;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

type BoxedFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;

// api call의 결과 외에도 계산 결과 저장도 겸임
pub struct Cache<T> {
  memory: DashMap<String, T>,
  storage_path: PathBuf,
}

impl<T: Clone + Send + Sync + 'static + Serialize> Cache<T> {
  pub fn new(storage_path: PathBuf) -> Self {
    Self {
      memory: DashMap::new(),
      storage_path,
    }
  }

  // memory cache failed, disk hit
  pub async fn get<F>(&self, key: &str, fetch_fn: F) -> Result<T>
  where
    F: FnOnce() -> BoxedFuture<Result<T>>,
  {
    if let Some(entry) = self.memory.get(key) {
      return Ok(entry.clone());
    }
    let value = fetch_fn().await?;
    self.memory.insert(key.to_string(), value.clone());
    Ok(value)
  }

  // 수동으로 메모리 overwrite
  pub fn set(&self, key: &str, value: T) -> Result<()> {
    self.memory.insert(key.to_string(), value);
    Ok(())
  }

  // 수동 플러시
  pub fn persist(&self) -> Result<()> {
    let mut map = HashMap::new();
    for entry in self.memory.iter() {
      map.insert(entry.key().clone(), entry.value().clone());
    }

    let json: _ = serde_json::to_string_pretty(&map)?;
    fs::write(&self.storage_path, json)?;

    Ok(())
  }

  // 수동 로드
  pub fn load(&self) -> Result<()>
  where
    T: for<'de> Deserialize<'de>,
  {
    if !self.storage_path.exists() {
      return Ok(()); // 파일이 없으면 그냥 넘어가기
    }

    let data = std::fs::read_to_string(&self.storage_path)?;
    if data.is_empty() {
      return Ok(());
    }

    let data = std::fs::read_to_string(&self.storage_path)?;
    let cached: HashMap<String, T> = serde_json::from_str(&data)?;
    for (key, value) in cached {
      self.memory.insert(key, value);
    }
    Ok(())
  }
}

pub fn get_cache_dir() -> Result<PathBuf> {
  if let Some(proj_dirs) = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
    let cache_dir = proj_dirs.cache_dir().to_path_buf();
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
  } else {
    Err(anyhow!("failed to get cache dir"))
  }
}
