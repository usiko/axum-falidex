use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Mutex;

/// Vérifie un token temporaire (valide 60 secondes)
/// - Vérifie que le timestamp n'est pas expiré
/// - Vérifie que le hash correspond à role+secret+timestamp
pub fn verify_temp_token(role: &str, timestamp: i64, hash_received: &str) -> bool {
    use sha2::{Digest, Sha256};
    let now = Utc::now().timestamp();
    // Vérifier expiration (60 secondes)
    if timestamp > now || now - timestamp > 60 {
        return false;
    }
    // Crée un hash SHA256 à partir de: role|secret|timestamp
    let secret = crate::env::get_token_hash_key();
    let data = format!("{}|{}|{}", role, secret, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let expected_hash = hex::encode(hasher.finalize());
    expected_hash == hash_received
}

/// Structure qui lie tokens et cookies pour gestion croisée
pub struct SecurityStore {
    // token_hash -> (cookie, expiration)
    tokens: Mutex<HashMap<String, (String, i64)>>,
    // cookie -> (token_hash, expiration)
    cookies: Mutex<HashMap<String, (String, i64)>>,
    // token_hash -> uuid (pour retrouver le UUID d'origine)
    hash_to_uuid: Mutex<HashMap<String, String>>,
}

impl SecurityStore {
    /// Crée un nouveau store vide
    pub fn new() -> Self {
        SecurityStore {
            tokens: Mutex::new(HashMap::new()),
            cookies: Mutex::new(HashMap::new()),
            hash_to_uuid: Mutex::new(HashMap::new()),
        }
    }

    /// Génère un token et un cookie liés, valides 24h, et les stocke
    /// Le token retourné au client est le UUID, mais le stockage utilise le hash dérivé
    pub fn generate_and_store(&self) -> (String, String) {
        let token_uuid = uuid::Uuid::new_v4().to_string();
        let token_hash = derivate_token(&token_uuid);
        let cookie = uuid::Uuid::new_v4().to_string();
        let expiration = Utc::now().timestamp() + 86400; // +24h
        self.tokens
            .lock()
            .unwrap()
            .insert(token_hash.clone(), (cookie.clone(), expiration));
        self.cookies
            .lock()
            .unwrap()
            .insert(cookie.clone(), (token_hash.clone(), expiration));
        self.hash_to_uuid
            .lock()
            .unwrap()
            .insert(token_hash.clone(), token_uuid.clone());
        (token_uuid, cookie)
    }

    /// Vérifie si un token (UUID) est valide et retourne le cookie associé si oui
    pub fn verify_token(&self, token: &str) -> Option<String> {
        let token_hash = derivate_token(token);
        if let Some((cookie, expiration)) = self.tokens.lock().unwrap().get(&token_hash) {
            if Utc::now().timestamp() < *expiration {
                return Some(cookie.clone());
            }
        }
        None
    }

    /// Vérifie si un cookie est valide et retourne le token UUID associé si oui
    pub fn verify_cookie(&self, cookie: &str) -> Option<String> {
        if let Some((token_hash, expiration)) = self.cookies.lock().unwrap().get(cookie) {
            if Utc::now().timestamp() < *expiration {
                // On retrouve le UUID d'origine à partir du hash
                if let Some(uuid) = self.hash_to_uuid.lock().unwrap().get(token_hash) {
                    return Some(uuid.clone());
                }
            }
        }
        None
    }

    /// Révoque un token (UUID ou hash) et le cookie associé
    pub fn revoke_by_token(&self, token: &str) {
        let token_hash = derivate_token(token);
        if let Some((cookie, _)) = self.tokens.lock().unwrap().remove(&token_hash) {
            self.cookies.lock().unwrap().remove(&cookie);
        }
        self.hash_to_uuid.lock().unwrap().remove(&token_hash);
    }

    /// Révoque un cookie et le token associé
    pub fn revoke_by_cookie(&self, cookie: &str) {
        if let Some((token_hash, _)) = self.cookies.lock().unwrap().remove(cookie) {
            self.tokens.lock().unwrap().remove(&token_hash);
            self.hash_to_uuid.lock().unwrap().remove(&token_hash);
        }
    }
}

fn derivate_token(token: &str) -> String {
    let secret = crate::env::get_derivated_token_hash_key();
    let data = format!("{}|{}", token, secret);
    let mut hasher = sha2::Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}
