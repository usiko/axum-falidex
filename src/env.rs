pub fn get_encrypt_key() -> String {
    match std::env::var("ENCRYPT_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️ ENCRYPT_KEY non définie, utilisation de la valeur par défaut");
            "default_dev_secret_please_change".to_string()
        }
    }
}

pub fn get_port() -> String {
    match std::env::var("PORT") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️ PORT non définie, utilisation de la valeur par défaut: 3000");
            "3000".to_string()
        }
    }
}

pub fn get_jwt_secret() -> String {
    match std::env::var("JWT_SECRET") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️ JWT_SECRET non définie, utilisation de la valeur par défaut");
            "default_dev_secret_please_change".to_string()
        }
    }
}

pub fn get_token_hash_key() -> String {
    match std::env::var("TOKEN_HASH_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️ TOKEN_HASH_KEY non définie, utilisation de la valeur par défaut");
            "default_dev_token_hash_please_change".to_string()
        }
    }
}

pub fn get_derivated_token_hash_key() -> String {
    match std::env::var("DERIVATE_TOKEN_HASH_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️ DERIVATE_TOKEN_HASH_KEY non définie, utilisation de la valeur par défaut");
            "default_dev_token_hash_please_change".to_string()
        }
    }
}

pub fn get_allowed_origins() -> String {
    match std::env::var("CORS_ORIGIN") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️ CORS_ORIGIN non définie, utilisation de la valeur par défaut: *");
            "*".to_string()
        }
    }
}
pub fn get_bdd() -> String {
    match std::env::var("BDD") {
        Ok(val) => val,
        Err(_) => {
            println!("BDD, utilisation de la valeur par défaut: ");
            "test-falidex".to_string()
        }
    }
}

pub fn get_app_resource_tag() -> String {
    return "test_falidex".to_string();
}
