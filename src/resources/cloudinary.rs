use crate::cache::FileCache;
use crate::env;
use crate::resources::model::CloudinaryUploadResponse;
use base64::prelude::*;
use chrono::Utc;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use reqwest::{Client, multipart};
use sha2::{Digest as Sha2Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
pub async fn upload_picture_for_symbole(
    file_bytes: Vec<u8>,
    filename: String,
    symbole_id: &str,
) -> Result<String, String> {
    let symbole_tag = format!("symbole-{}", symbole_id);
    let tags = Vec::from(["symbole".to_string(), symbole_tag.clone()]);
    //attributes.
    let folder = format!("{}/{}/{}", get_app_tag(), "symbole", symbole_tag);
    let result = upload_picture(
        file_bytes,
        filename,
        "falidex".to_string(),
        folder.clone(),
        tags,
    )
    .await;
    match result {
        Ok(response) => {
            if let Some(id) = &response.public_id {
                // Met à jour le cache du dossier concerné
                // Reset le cache du symbole (clé = symbole_id)
                let cache = FileCache::new(".app_temp/urls");
                let _ = cache.delete(symbole_id).await;
                Ok(id.clone())
            } else {
                Err("Cloudinary: public_id manquant dans la réponse".to_string())
            }
        }
        Err(e) => Err(format!("Erreur HTTP: {}", e)),
    }
    //upload_data_url(data_url, tags).await // temp
}

pub async fn remove_picture() {}

pub async fn get_asset_in_folder(folder: &str) -> Result<Vec<String>, String> {
    // Not in cache or expired, fetch from Cloudinary
    let cloud_name = get_cloud_name();
    let api_key = get_api_key();
    let api_secret = get_api_key_secret();
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/resources/search",
        cloud_name
    );
    let expression = format!("folder:{}", folder);
    #[derive(serde::Serialize)]
    struct SearchBody<'a> {
        expression: &'a str,
    }
    let client = Client::new();
    let res = client
        .post(&url)
        .basic_auth(api_key, Some(api_secret))
        .json(&SearchBody {
            expression: &expression,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = res.status();
    let text = res.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("Cloudinary search error: {}", text));
    }
    #[derive(serde::Deserialize)]
    struct CloudinarySearchResponse {
        resources: Vec<CloudinaryAsset>,
    }
    #[derive(serde::Deserialize)]
    struct CloudinaryAsset {
        public_id: String,
    }
    let parsed: CloudinarySearchResponse =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let ids: Vec<String> = parsed
        .resources
        .into_iter()
        .map(|asset| {
            // Ne garder que le dernier segment de l'id (après le dernier '/')
            utf8_percent_encode(&asset.public_id, NON_ALPHANUMERIC).to_string()
        })
        .collect();

    Ok(ids)
}

pub async fn get_urls_for_symbole(symbole_id: String) -> Result<Vec<String>, String> {
    // Garde-fou anti-rate-limit : on ne traite que symbole-112
    /*if symbole_id != "symbole-112" {
        return Ok(Vec::new());
    }*/
    let cache = FileCache::new(".app_temp/urls");
    if let Ok(Some(urls)) = cache.get::<Vec<String>>(&symbole_id).await {
        return Ok(urls);
    }
    // Si pas en cache, calcule et stocke
    let folder = format!("{}/{}/symbole-{}", get_app_tag(), "symbole", symbole_id);
    let result = get_asset_in_folder(&folder).await;
    println!("recherche d'img pour {}", symbole_id);
    match result {
        Ok(tags) => {
            let urls: Vec<String> = tags.iter().map(|id| format!("/resource/{}", id)).collect();
            if urls.is_empty() {
                Err("Aucune image trouvée pour ce symbole".to_string())
            } else {
                // Met en cache pour 1h (3600s)
                let _ = cache.set(&symbole_id, &urls, 3600).await;
                println!("success getting picture {:?}{}", urls.clone(), symbole_id);
                Ok(urls)
            }
        }
        Err(error) => {
            println!("error cloudinary {}", error);
            Err(error)
        }
    }
}
pub async fn get_picture(id_picture: String) -> Result<Vec<u8>, String> {
    let cache = FileCache::new(".app_temp/pictures");
    let cache_key = utf8_percent_encode(&id_picture, NON_ALPHANUMERIC).to_string();
    if let Ok(Some(bytes)) = cache.get::<Vec<u8>>(&cache_key).await {
        return Ok(bytes);
    }
    let url = get_delivery_url(
        id_picture.clone(),
        Some(vec!["f_auto".to_string(), "q_auto".to_string()]),
    );
    println!("cloudinary url for symbole {}", &url);
    match reqwest::get(&url).await {
        Ok(resp) if resp.status().is_success() => match resp.bytes().await {
            Ok(bytes) => {
                let bytes_vec = bytes.to_vec();
                let _ = cache.set(&cache_key, &bytes_vec, 3600).await;
                Ok(bytes_vec)
            }
            Err(e) => Err(format!("Erreur lecture image: {}", e)),
        },
        Ok(resp) => {
            eprintln!("Cloudinary error: status {} for {}", resp.status(), url);
            Err("Image not found or inaccessible".to_string())
        }
        Err(e) => {
            eprintln!("Cloudinary request failed: {} for {}", e, url);
            Err("Image not found or inaccessible".to_string())
        }
    }
}

pub fn get_delivery_url(id: String, attributs: Option<Vec<String>>) -> String {
    let param_url = get_delivery_param_url(id.clone(), attributs.clone());
    let signature = get_delivery_signature(param_url.clone());
    format!(
        "https://res.cloudinary.com/{}/image/authenticated/{}/{}",
        get_cloud_name(),
        signature,
        param_url
    )
}

fn get_api_key() -> String {
    "689958834963682".to_string()
}

fn get_api_key_secret() -> String {
    "2P388YE0UgTXbYUi_3Mip8Q6_FQ".to_string()
}

fn get_cloud_name() -> String {
    "dbtqrsibv".to_string()
}

fn get_app_tag() -> String {
    env::get_app_resource_tag()
}

/**
 * generate signature for cloudinary
 */
fn get_signature_upload(attribut_to_send: Option<HashMap<String, String>>) -> String {
    let not_allowed_keys = std::collections::HashSet::from([
        "file",
        "cloud_name",
        "api_key",
        "signature",
        "resource_type",
    ]);
    let attributes_string = attribut_to_send
        .map(|item| {
            let mut keys: Vec<_> = item
                .iter()
                .filter(|(key, _)| !not_allowed_keys.contains(key.as_str()))
                .collect();
            keys.sort_by_key(|(key, _)| key.clone());
            keys.into_iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join("&")
        })
        .unwrap_or_default();
    let to_serialize = format!("{}{}", attributes_string, get_api_key_secret());
    let mut hasher = Sha256::new();
    hasher.update(to_serialize.as_bytes());
    hex::encode(hasher.finalize())
}

fn get_delivery_param_url(id: String, attributs: Option<Vec<String>>) -> String {
    println!("get delivery param url {},{:?}", &id, &attributs);
    let attributes_string = attributs.unwrap_or_default().join("/");
    let decoded_id = percent_decode_str(&id)
        .decode_utf8()
        .expect("Invalid UTF-8")
        .to_string();
    if attributes_string.is_empty() {
        let url = format!("{}", decoded_id);
        println!("delivery param url {}", &url);
        url
    } else {
        let url = format!("{}/{}", attributes_string, decoded_id);
        println!("delivery param url {}", &url);
        url
    }
}
fn get_delivery_signature(param_url_delivery: String) -> String {
    let mut hasher = Sha256::new();
    // Cloudinary attend que le timestamp soit dans la chaîne à signer
    let to_sign = format!("{}{}", param_url_delivery, get_api_key_secret());
    println!("get delivery signature {}", &to_sign);
    hasher.update(to_sign.as_bytes());

    let hash = BASE64_STANDARD.encode(hasher.finalize());
    println!("get delivery hash {}", &hash);
    format!("s--{}--", hash.chars().take(8).collect::<String>())
}

async fn upload_picture(
    file_bytes: Vec<u8>,
    filename: String,
    preset: String,
    folder: String,
    tags: Vec<String>,
) -> Result<CloudinaryUploadResponse, String> {
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        get_cloud_name()
    );
    let timestamp = Utc::now().timestamp();
    let client = Client::new();
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("upload_preset".to_string(), preset);
    attributes.insert("folder".to_string(), folder);
    attributes.insert("tags".to_string(), tags.join(","));
    attributes.insert("api_key".to_string(), get_api_key());
    attributes.insert("timestamp".to_string(), timestamp.to_string());
    let signature = get_signature_upload(Some(attributes.clone()));
    attributes.insert("signature".to_string(), signature);
    let multipart = multipart::Part::bytes(file_bytes).file_name(filename);
    let mut form = multipart::Form::new().part("file", multipart);
    for (key, value) in &attributes {
        form = form.text(key.clone(), value.clone());
    }
    println!("upload to cloudinary {} {:?}", &url, attributes);
    let response = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let text = response.text().await.map_err(|e| e.to_string())?;
    let parsed: CloudinaryUploadResponse =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if let Some(err) = &parsed.error {
        Err(err.message.clone())
    } else {
        Ok(parsed)
    }
}
