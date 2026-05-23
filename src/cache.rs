/// Module de cache générique avec persistance sur disque et expiration automatique.
/// Utilise un HashMap en mémoire et sérialise en JSON sur disque.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Entrée du cache, stocke la valeur et la date d'expiration (timestamp en secondes).
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheEntry<V> {
    /// Valeur stockée
    value: V,
    /// Timestamp d'expiration (secondes depuis l'époque Unix)
    expires_at: u64,
}

/// Cache générique clé/valeur avec expiration et persistance sur disque.
/// K : type de la clé (doit être sérialisable et hashable)
/// V : type de la valeur (doit être sérialisable)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FileCache<K, V>
where
    K: Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    /// Map interne clé -> (valeur, expiration)
    map: HashMap<K, CacheEntry<V>>,
    /// Durée de vie (en secondes) des entrées du cache
    ttl_secs: u64,
}

impl<K, V> FileCache<K, V>
where
    K: Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    /// Crée un nouveau cache avec une durée de vie (en secondes) pour chaque entrée.
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            map: HashMap::new(),
            ttl_secs,
        }
    }

    /// Récupère une valeur du cache si elle existe et n'est pas expirée.
    /// Supprime l'entrée si elle est expirée.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let now = now_epoch();
        if let Some(entry) = self.map.get(key) {
            if entry.expires_at > now {
                return Some(&entry.value);
            }
        }
        self.map.remove(key);
        None
    }

    /// Ajoute ou met à jour une valeur dans le cache, avec expiration automatique.
    pub fn set(&mut self, key: K, value: V) {
        let expires_at = now_epoch() + self.ttl_secs;
        self.map.insert(key, CacheEntry { value, expires_at });
    }

    /// Supprime toutes les entrées expirées du cache.
    pub fn clean_expired(&mut self) {
        let now = now_epoch();
        self.map.retain(|_, entry| entry.expires_at > now);
    }

    /// Sauvegarde le cache sur disque au format JSON.
    /// Ecrase le fichier s'il existe déjà.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    /// Charge un cache depuis un fichier JSON. Si le fichier n'existe pas, crée un cache vide.
    /// Nettoie les entrées expirées au chargement.
    pub fn load_from_file<P: AsRef<Path>>(path: P, ttl_secs: u64) -> io::Result<Self> {
        if !path.as_ref().exists() {
            return Ok(Self::new(ttl_secs));
        }
        let data = fs::read_to_string(path)?;
        let mut cache: Self = serde_json::from_str(&data)?;
        cache.ttl_secs = ttl_secs;
        cache.clean_expired();
        Ok(cache)
    }
}

/// Renvoie le timestamp actuel en secondes depuis l'époque Unix.
fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
// to do

// implement cache here and persist here to files
