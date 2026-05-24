use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
struct CacheEntry<T> {
    value: T,
    expires_at: u64,
}

pub struct FileCache {
    base_path: PathBuf,
}

impl FileCache {
    pub fn new(base_path: &str) -> Self {
        let path = PathBuf::from(base_path);
        std::fs::create_dir_all(&path).ok();
        Self { base_path: path }
    }

    fn file_path(&self, key: &str) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(format!("{}.json", key));
        path
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    // SET cache avec TTL
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: T,
        ttl_secs: u64,
    ) -> Result<(), String> {
        let entry = CacheEntry {
            value,
            expires_at: Self::now() + ttl_secs,
        };

        let json = serde_json::to_string(&entry).map_err(|e| e.to_string())?;

        fs::write(self.file_path(key), json)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    // GET cache avec expiration auto
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, String> {
        let path = self.file_path(key);

        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&path).await.map_err(|e| e.to_string())?;

        let entry: CacheEntry<T> = match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(_) => {
                let _ = fs::remove_file(&path).await;
                return Ok(None);
            }
        };

        if Self::now() > entry.expires_at {
            let _ = fs::remove_file(&path).await;
            return Ok(None);
        }

        Ok(Some(entry.value))
    }

    // DELETE manuel
    pub async fn delete(&self, key: &str) -> Result<(), String> {
        let _ = fs::remove_file(self.file_path(key)).await;
        Ok(())
    }

    // CLEAN global (optionnel)
    pub async fn clean_expired(&self) -> Result<(), String> {
        let mut dir = fs::read_dir(&self.base_path)
            .await
            .map_err(|e| e.to_string())?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| e.to_string())? {
            let path = entry.path();

            if let Ok(data) = fs::read_to_string(&path).await {
                let parsed: Result<CacheEntry<serde_json::Value>, _> = serde_json::from_str(&data);

                if let Ok(entry) = parsed {
                    if Self::now() > entry.expires_at {
                        let _ = fs::remove_file(path).await;
                    }
                }
            }
        }

        Ok(())
    }
}
