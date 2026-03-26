use base64::{Engine as _, engine::general_purpose};
use simple_crypt::{decrypt, encrypt};
pub fn encrypt_data(data: String) -> Result<String, String> {
    let key = get_encrypt_key();
    let encrypted = encrypt(data.as_bytes(), key.as_bytes())
        .map_err(|e| e.to_string())
        .map(|result| encrypt_bytes_to_b64(&result))?;
    Ok(encrypted)
}

pub fn decrypt_data(encrypted_data: String) -> Result<String, String> {
    let key = get_encrypt_key();
    let bytes = decrypt_b64_to_bytes(encrypted_data)?;
    decrypt(&bytes, key.as_bytes())
        .map_err(|e| e.to_string())
        .and_then(|result| bytes_to_string(&result))
}

fn get_encrypt_key() -> String {
    std::env::var("ENCRYPT_KEY").unwrap_or("default_dev_secret_please_change".to_string())
}

fn bytes_to_string(data: &[u8]) -> Result<String, String> {
    String::from_utf8(data.to_vec()).map_err(|e| e.to_string())
}

fn encrypt_bytes_to_b64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}
fn decrypt_b64_to_bytes(data: String) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| e.to_string())
}
