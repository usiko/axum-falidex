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
    /// Filtre toutes les listes du cache (toutes les clés) selon un prédicat, et met à jour ou supprime la clé si la liste devient vide
    pub async fn filter_all_lists<T, F>(&self, mut predicate: F) -> Result<(), String>
    where
        T: for<'de> Deserialize<'de> + Serialize,
        F: FnMut(&T) -> bool,
    {
        let mut dir = fs::read_dir(&self.base_path)
            .await
            .map_err(|e| e.to_string())?;
        while let Some(entry) = dir.next_entry().await.map_err(|e| e.to_string())? {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.ends_with(".json") {
                    let key = filename.trim_end_matches(".json");
                    if let Some(mut list) = self.get::<Vec<T>>(key).await? {
                        let original_len = list.len();
                        list.retain(|item| predicate(item));
                        if list.len() != original_len {
                            if list.is_empty() {
                                let _ = self.delete(key).await;
                            } else {
                                let _ = self.set(key, list, 3600).await;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    /// Filtre une liste stockée en cache (clé) selon un prédicat, et met à jour ou supprime la clé si la liste devient vide
    pub async fn filter_list<T, F>(&self, key: &str, mut predicate: F) -> Result<(), String>
    where
        T: for<'de> Deserialize<'de> + Serialize,
        F: FnMut(&T) -> bool,
    {
        if let Some(mut list) = self.get::<Vec<T>>(key).await? {
            let original_len = list.len();
            list.retain(|item| predicate(item));
            if list.len() != original_len {
                if list.is_empty() {
                    let _ = self.delete(key).await;
                } else {
                    let _ = self.set(key, list, 3600).await;
                }
            }
        }
        Ok(())
    }
    /// Recherche une entrée dans le cache selon un prédicat sur la valeur (fallback générique)
    pub async fn find_by<T, F>(&self, mut predicate: F) -> Result<Option<T>, String>
    where
        T: for<'de> Deserialize<'de>,
        F: FnMut(&T) -> bool,
    {
        let mut dir = fs::read_dir(&self.base_path)
            .await
            .map_err(|e| e.to_string())?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| e.to_string())? {
            let path = entry.path();
            // Ignore les fichiers qui ne sont pas des .json
            if let Some(ext) = path.extension() {
                if ext != "json" {
                    continue;
                }
            }
            let data = match fs::read_to_string(&path).await {
                Ok(d) => d,
                Err(_) => continue,
            };
            let entry: CacheEntry<T> = match serde_json::from_str(&data) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if Self::now() > entry.expires_at {
                let _ = fs::remove_file(&path).await;
                continue;
            }
            if predicate(&entry.value) {
                return Ok(Some(entry.value));
            }
        }
        Ok(None)
    }
    // Supprime toutes les entrées dont la clé contient la sous-chaîne donnée
    pub async fn delete_partial(&self, partial_key: &str) -> Result<(), String> {
        let mut dir = fs::read_dir(&self.base_path)
            .await
            .map_err(|e| e.to_string())?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| e.to_string())? {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.contains(partial_key) {
                    let _ = fs::remove_file(&path).await;
                }
            }
        }
        Ok(())
    }
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
