use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Mutex;

// ============================================================
// ÉTAPE 1: VÉRIFICATION TOKEN TEMPORAIRE (role+secret+timestamp)
// ============================================================

/// Crée un hash SHA256 à partir de: role|secret|timestamp
fn hash_token(role: &str, timestamp: i64) -> String {
    let secret = crate::env::get_token_hash_key();
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
    if timestamp > now || now - timestamp > 60 {
        return false;
    }
    // Vérifier le hash
    let expected_hash = hash_token(role, timestamp);
    expected_hash == hash_received
}

fn derivate_token(token: &str) -> String {
    let secret = crate::env::get_derivated_token_hash_key();
    let data = format!("{}|{}", token, secret);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

// ============================================================
// ÉTAPE 2: GÉNÉRATION ET STOCKAGE TOKEN LONG TERME
// ============================================================

/// Stockage en mémoire des tokens valides
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityStoreAutorization {
    pub expiration: i64,
    pub token: String,
    pub derivated_token: String,
    pub cookie: String,
}

pub struct SecurityStore {
    autorization: Mutex<HashMap<String, SecurityStoreAutorization>>,
    derivated_token_to_id: Mutex<HashMap<String, String>>, // token -> id
    cookie_to_id: Mutex<HashMap<String, String>>,          // cookie -> id
}

impl SecurityStore {
    /// Crée un nouveau store vide
    pub fn new() -> Self {
        SecurityStore {
            autorization: Mutex::new(HashMap::new()),
            derivated_token_to_id: Mutex::new(HashMap::new()), // token -> id
            cookie_to_id: Mutex::new(HashMap::new()),          // cookie -> id
        }
    }

    /// Génère un nouveau token UUID valide 24h et le stocke
    /// le uuid est donné au front mais ce n'est pas lui qui sert de token
    /// le token valid est une derivation du uuid, un hash token+key
    pub fn generate_and_store(&self) -> SecurityStoreAutorization {
        let token = uuid::Uuid::new_v4().to_string();
        let cookie = uuid::Uuid::new_v4().to_string();
        let derivated_token = derivate_token(&token);
        let expiration = Utc::now().timestamp() + 86400; // +24h

        let autorisation = SecurityStoreAutorization {
            token,
            derivated_token,
            expiration,
            cookie,
        };
        self.add_autorisation(autorisation.clone());

        autorisation
    }

    /// Vérifie si un token stocké est encore valide
    pub fn verify_stored_token(&self, token: &str) -> bool {
        if let Some(autorisation) = self.get_autorisaton_by_derivated_token(token) {
            if (Utc::now().timestamp() < autorisation.expiration) {
                return true;
            }
            self.revoke_token(token);
        }
        false
    }

    /// Supprime un token (logout)
    pub fn revoke_token(&self, derivated_token: &str) {
        // Trouver l'id associé au token dérivé
        let id_opt = self
            .derivated_token_to_id
            .lock()
            .unwrap()
            .get(derivated_token)
            .cloned();
        if let Some(id) = id_opt {
            // Récupérer l'autorisation pour obtenir le cookie
            let cookie_opt = self
                .autorization
                .lock()
                .unwrap()
                .get(&id)
                .map(|a| a.cookie.clone());
            // Supprimer dans autorization
            self.autorization.lock().unwrap().remove(&id);
            // Supprimer dans derivated_token_to_id
            self.derivated_token_to_id
                .lock()
                .unwrap()
                .remove(derivated_token);
            // Supprimer dans cookie_to_id
            if let Some(cookie) = cookie_opt {
                self.cookie_to_id.lock().unwrap().remove(&cookie);
            }
        }
    }

    /// Supprime un cookie (logout)
    pub fn revoke_cookie(&self, cookie: &str) {
        // Trouver l'id associé au cookie
        let id_opt = self.cookie_to_id.lock().unwrap().get(cookie).cloned();
        if let Some(id) = id_opt {
            // Récupérer l'autorisation pour obtenir le token dérivé
            let derivated_token_opt = self
                .autorization
                .lock()
                .unwrap()
                .get(&id)
                .map(|a| a.derivated_token.clone());
            // Supprimer dans autorization
            self.autorization.lock().unwrap().remove(&id);
            // Supprimer dans cookie_to_id
            self.cookie_to_id.lock().unwrap().remove(cookie);
            // Supprimer dans derivated_token_to_id
            if let Some(derivated_token) = derivated_token_opt {
                self.derivated_token_to_id
                    .lock()
                    .unwrap()
                    .remove(&derivated_token);
            }
        }
    }

    // ===================== GESTION DES COOKIES =====================

    /// Vérifie si un cookie stocké est encore valide
    pub fn verify_stored_cookie(&self, cookie: &str) -> bool {
        if let Some(autorisation) = self.get_autorisaton_by_cookie(&cookie) {
            if (Utc::now().timestamp() < autorisation.expiration) {
                return true;
            }
            self.revoke_cookie(cookie);
        }
        false
    }

    fn add_autorisation(&self, autorisation: SecurityStoreAutorization) {
        let id = uuid::Uuid::new_v4().to_string();
        self.autorization
            .lock()
            .unwrap()
            .insert(id.clone(), autorisation.clone());
        self.derivated_token_to_id
            .lock()
            .unwrap()
            .insert(autorisation.derivated_token.clone(), id.clone());
        self.cookie_to_id
            .lock()
            .unwrap()
            .insert(autorisation.cookie.clone(), id.clone());
        //self.derivated_token_to_id
    }

    fn get_autorisaton_by_derivated_token(
        &self,
        derivated_token: &str,
    ) -> Option<SecurityStoreAutorization> {
        let res = self
            .derivated_token_to_id
            .lock()
            .unwrap()
            .get(derivated_token)
            .cloned();
        match res {
            Some(id) => return self.autorization.lock().unwrap().get(&id).cloned(),
            None => None,
        }
    }
    fn get_autorisaton_by_cookie(&self, cookie: &str) -> Option<SecurityStoreAutorization> {
        let res = self.cookie_to_id.lock().unwrap().get(cookie).cloned();
        match res {
            Some(id) => return self.autorization.lock().unwrap().get(&id).cloned(),
            None => None,
        }
    }
    pub fn clear_expired(&self) {
        let now = Utc::now().timestamp();

        // Collecter les ids expirés
        let expired_ids: Vec<String> = self
            .autorization
            .lock()
            .unwrap()
            .iter()
            .filter_map(|(id, auth)| {
                if auth.expiration < now {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();

        // Supprimer dans tous les index pour chaque id expiré
        for id in expired_ids {
            if let Some(auth) = self.autorization.lock().unwrap().remove(&id) {
                self.derivated_token_to_id
                    .lock()
                    .unwrap()
                    .remove(&auth.derivated_token);
                self.cookie_to_id.lock().unwrap().remove(&auth.cookie);
            }
        }
    }
}
