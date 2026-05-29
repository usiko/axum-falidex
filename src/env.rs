pub fn get_encrypt_key() -> String {
    get_env_value("ENCRYPT_KEY", None)
}

pub fn get_port() -> String {
    get_env_value("PORT", Some("3000".to_string()))
}

pub fn get_jwt_secret() -> String {
    get_env_value("JWT_SECRET", None)
}

pub fn get_token_hash_key() -> String {
    get_env_value("TOKEN_HASH_KEY", None)
}

pub fn get_derivated_token_hash_key() -> String {
    get_env_value("DERIVATE_TOKEN_HASH_KEY", None)
}

pub fn get_allowed_origins() -> String {
    get_env_value("CORS_ORIGIN", Some("*".to_string()))
}
pub fn get_bdd() -> String {
    get_env_value("BDD", Some("test-falidex".to_string()))
}

pub fn get_cloudinary_main_folder() -> String {
    get_env_value("CLOUDINARY_MAIN_FOLDER", Some("test_falidex".to_string()))
}
pub fn get_cloudinary_api_key() -> String {
    get_env_value("CLOUDINARY_API_KEY", None)
}
pub fn get_cloudinary_api_secret_key() -> String {
    get_env_value("CLOUDINARY_API_SECRET_KEY", None)
}
pub fn get_cloudinary_cloud_name() -> String {
    get_env_value("CLOUDINARY_CLOUD_NAME", None)
}

pub fn is_migration_img_activated() -> bool {
    let value = get_env_value("MIGRATION_IMG", Some("false".to_string()));
    value == "true"
}

pub fn is_fix_relation_id_activated() -> bool {
    let value = get_env_value("FIX_RELATION_ID", Some("false".to_string()));
    value == "true"
}

fn get_env_value(key: &str, default: Option<String>) -> String {
    match default {
        Some(default_val) => match std::env::var(&key) {
            Ok(val) => val,
            Err(_) => {
                println!(
                    "⚠️ {} non définie, utilisation de la valeur par défaut: {}",
                    &key, default_val
                );
                default_val
            }
        },
        None => {
            let error_message = format!("⚠️ {} non définie", key);
            std::env::var(key).expect(&error_message)
        }
    }
}
