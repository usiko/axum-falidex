use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Mutex;

// ============================================================
// ÉTAPE 1: VÉRIFICATION TOKEN TEMPORAIRE (role+secret+timestamp)
// ============================================================

/// Crée un hash SHA256 à partir de: role|secret|timestamp
fn hash_token(role: &str, timestamp: i64) -> String {
    let secret = get_token_hash_key();
    let data = format!("{}|{}|{}", role, secret, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn show_dev_ex_token(role: &str) {
    let now = Utc::now().timestamp();
    println!("test token {}, timestamp {}", hash_token(role, now), now);
}

/// Vérifie un token temporaire (valide 60 secondes)
/// - Vérifie que le timestamp n'est pas expiré
/// - Vérifie que le hash correspond à role+secret+timestamp
pub fn verify_temp_token(role: &str, timestamp: i64, hash_received: &str) -> bool {
    let now = Utc::now().timestamp();

    // Vérifier expiration (60 secondes)
    if (now - timestamp).abs() > 60 {
        return false;
    }

    // Vérifier le hash
    let expected_hash = hash_token(role, timestamp);
    expected_hash == hash_received
}
fn get_token_hash_key() -> String {
    std::env::var("TOKEN_HASH_KEY").unwrap_or("default_dev_token_hash_please_change".to_string())
}

// ============================================================
// ÉTAPE 2: GÉNÉRATION ET STOCKAGE TOKEN LONG TERME
// ============================================================

/// Stockage en mémoire des tokens valides
pub struct TokenStore {
    tokens: Mutex<HashMap<String, i64>>, // token -> timestamp d'expiration
}

impl TokenStore {
    /// Crée un nouveau store vide
    pub fn new() -> Self {
        TokenStore {
            tokens: Mutex::new(HashMap::new()),
        }
    }

    /// Génère un nouveau token UUID valide 24h et le stocke
    pub fn generate_and_store(&self) -> String {
        let token = uuid::Uuid::new_v4().to_string();
        let expiration = Utc::now().timestamp() + 86400; // +24h

        self.tokens
            .lock()
            .unwrap()
            .insert(token.clone(), expiration);
        token
    }

    /// Vérifie si un token stocké est encore valide
    pub fn verify_stored_token(&self, token: &str) -> bool {
        if let Some(&expiration) = self.tokens.lock().unwrap().get(token) {
            return Utc::now().timestamp() < expiration;
        }
        false
    }

    /// Supprime un token (logout)
    pub fn revoke(&self, token: &str) {
        self.tokens.lock().unwrap().remove(token);
    }
}
