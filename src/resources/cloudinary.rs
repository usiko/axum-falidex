use crate::env;
use crate::resources::model::{
    CloudinaryAsset, CloudinaryAssetWithId, CloudinarySearchResponse, CloudinaryUploadResponse,
    DeleteParams,
};
use crate::{cache::FileCache, resources::model::AssetUrl};
use base64::prelude::*;
use chrono::Utc;
use reqwest::{Client, multipart};
use sha2::{Digest as Sha2Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use uuid::Uuid;
//
pub async fn upload_picture_for_symbole(
    file_bytes: Vec<u8>,
    filename: String,
    symbole_id: &str,
) -> Result<AssetUrl, String> {
    let symbole_tag = format!("symbole-{}", symbole_id);
    let tags = Vec::from(["symbole".to_string(), symbole_tag.clone()]);
    //attributes.
    let folder = format!("{}/{}/{}", get_app_tag(), "symbole", symbole_tag);
    let result = upload_file_picture(
        file_bytes,
        filename,
        "falidex".to_string(),
        folder.clone(),
        tags,
    )
    .await;
    match result {
        Ok(response) => update_cache_after_upload(response, &symbole_id, true).await,
        Err(e) => Err(format!("Erreur HTTP: {}", e)),
    }
    //upload_data_url(data_url, tags).await // temp
}

pub async fn remove_picture(local_id: String) -> Result<String, String> {
    let asset_id = get_asset_id_by_local_id(&local_id)
        .await
        .ok_or_else(|| "asset_id introuvable pour ce local_id".to_string())?;
    let timestamp = Utc::now().timestamp();
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/asset/destroy",
        get_cloud_name()
    );
    let client = Client::new();
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("asset_id".to_string(), asset_id.clone());
    attributes.insert("api_key".to_string(), get_api_key());
    attributes.insert("timestamp".to_string(), timestamp.to_string());
    let params = DeleteParams {
        asset_id: asset_id.clone(),
        api_key: get_api_key(),
        signature: get_write_signature(Some(attributes)),
        timestamp: timestamp.to_string(),
    };
    let res = client
        .post(&url)
        .json(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = res.status();
    let text = res.text().await.unwrap_or_default();
    let json_value: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|_| serde_json::json!({"error": "invalid json"}));
    println!(
        "delete picture response: {} {:?} {:?}",
        &url, params, json_value
    );
    let is_not_found = json_value
        .get("result")
        .and_then(|v| v.as_str())
        .map(|s| s == "not found")
        .unwrap_or(false);
    if is_not_found {
        return Err("not found".to_string());
    }
    if status.is_success() && !is_not_found {
        // Suppression des caches associés
        let cache_urls = FileCache::new(".app_temp/urls");
        let cache_delivery = FileCache::new(".app_temp/delivery_url");
        let cache_pictures = FileCache::new(".app_temp/pictures");
        let asset_cache_id = FileCache::new(".app_temp/asset_cache_id");
        let asset_cache_asset_id = FileCache::new(".app_temp/asset_cache_asset_id");

        // Nettoie le cache d'URL : retire toutes les AssetUrl dont id == local_id dans toutes les entrées
        let _ = cache_urls
            .filter_all_lists::<AssetUrl, _>(|a| a.id != local_id)
            .await;
        let _ = cache_delivery.delete_partial(&local_id).await;
        let _ = cache_pictures.delete(&local_id).await;

        let asset_id = get_asset_id_by_local_id(&local_id)
            .await
            .ok_or_else(|| "asset_id introuvable pour ce local_id".to_string())?;
        let _ = asset_cache_id.delete(&local_id).await;
        let _ = asset_cache_asset_id.delete(&asset_id).await;

        Ok(format!("Image supprimée: {}", local_id))
    } else {
        Err(format!(
            "Erreur suppression Cloudinary ({}): {json_value:?}",
            status
        ))
    }
}

pub async fn get_asset_in_folder(folder: &str) -> Result<Vec<CloudinaryAssetWithId>, String> {
    //
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

    let parsed: CloudinarySearchResponse =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let asset_cache_asset_id = FileCache::new(".app_temp/asset_cache_asset_id");
    let asset_cache_id = FileCache::new(".app_temp/asset_cache_id");

    let mut assets = Vec::new();
    for asset in parsed.resources.iter() {
        let existing = asset_cache_asset_id
            .get::<CloudinaryAssetWithId>(&asset.asset_id)
            .await
            .ok()
            .flatten();
        let cloudinary_asset = gen_cloudinary_asset(
            asset.asset_id.clone(),
            asset.public_id.clone(),
            existing.as_ref().map(|e| e.id.clone()),
        );

        // Par asset_id

        let _ = asset_cache_asset_id
            .set(&cloudinary_asset.asset_id, &cloudinary_asset, 24 * 3600)
            .await;
        // Par id (uuid)
        let _ = asset_cache_id
            .set(&cloudinary_asset.id, &cloudinary_asset, 24 * 3600)
            .await;
        assets.push(cloudinary_asset);
    }
    Ok(assets)
}

pub async fn get_urls_for_symbole(symbole_id: &str) -> Result<Vec<AssetUrl>, String> {
    // Garde-fou anti-rate-limit : on ne traite que symbole-112
    /*if symbole_id != "symbole-112" {
        return Ok(Vec::new());
    }*/
    let cache = FileCache::new(".app_temp/urls");
    if let Ok(Some(urls)) = cache.get::<Vec<AssetUrl>>(&symbole_id).await {
        return Ok(urls);
    }
    // Si pas en cache, calcule et stocke
    let folder = format!("{}/{}/symbole-{}", get_app_tag(), "symbole", symbole_id);
    let result = get_asset_in_folder(&folder).await;
    println!("recherche d'img pour {}", symbole_id);
    match result {
        Ok(tags) => {
            let urls: Vec<AssetUrl> = tags
                .iter()
                .map(|asset| get_asset_url_from_local_id(asset.id.clone()))
                .collect();
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
pub async fn get_picture(local_id: String, height: u16, width: u16) -> Result<Vec<u8>, String> {
    let asset_id = get_asset_id_by_local_id(&local_id)
        .await
        .ok_or_else(|| "asset_id introuvable pour ce local_id".to_string())?;
    let cache = FileCache::new(".app_temp/pictures");
    if let Ok(Some(bytes)) = cache.get::<Vec<u8>>(&local_id).await {
        return Ok(bytes);
    }
    let url = get_delivery_url(
        asset_id.clone(),
        Some(vec![
            format!("w_{}", width),
            format!("h_{}", height),
            "f_auto".to_string(),
            "q_auto".to_string(),
        ]),
    )
    .await
    .ok_or_else(|| "erreur url".to_string())?;
    println!("cloudinary url for symbole {}", &url);
    match reqwest::get(&url).await {
        Ok(resp) if resp.status().is_success() => match resp.bytes().await {
            Ok(bytes) => {
                let bytes_vec = bytes.to_vec();
                let _ = cache.set(&local_id, &bytes_vec, 3600).await;
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

/// Retourne l'URL de livraison Cloudinary, avec cache persistant (clé hashée, TTL 1h)
pub async fn get_delivery_url(local_id: String, attributs: Option<Vec<String>>) -> Option<String> {
    let public_id = get_public_id_by_local_id(&local_id).await?;
    let cache = FileCache::new(".app_temp/delivery_url");
    if let Ok(Some(url)) = cache.get::<String>(&local_id).await {
        return Some(url);
    }
    let param_url = get_delivery_param_url(public_id.clone(), attributs.clone());
    let signature = get_delivery_signature(param_url.clone());
    let url = format!(
        "https://res.cloudinary.com/{}/image/authenticated/{}/{}",
        get_cloud_name(),
        signature,
        param_url
    );
    let _ = cache.set(&local_id, &url, 3600).await;
    Some(url)
}

pub async fn get_public_id_by_asset_id(asset_id: &str) -> Option<String> {
    let cache = FileCache::new(".app_temp/asset_cache_asset_id");
    if let Ok(Some(asset)) = cache.get::<CloudinaryAssetWithId>(asset_id).await {
        return Some(asset.public_id);
    }
    None
}
pub async fn get_public_id_by_local_id(id: &str) -> Option<String> {
    let cache = FileCache::new(".app_temp/asset_cache_id");
    if let Ok(Some(asset)) = cache.get::<CloudinaryAssetWithId>(id).await {
        return Some(asset.public_id);
    }
    None
}
pub async fn get_asset_id_by_local_id(id: &str) -> Option<String> {
    let cache = FileCache::new(".app_temp/asset_cache_id");
    if let Ok(Some(asset)) = cache.get::<CloudinaryAssetWithId>(id).await {
        return Some(asset.asset_id);
    }
    None
}

/**
 * generate signature for cloudinary
 */
fn get_write_signature(attribut_to_send: Option<HashMap<String, String>>) -> String {
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
    if attributes_string.is_empty() {
        let url = format!("{}", id);
        println!("delivery param url {}", &url);
        url
    } else {
        let url = format!("{}/{}", attributes_string, id);
        println!("delivery param url {}", &url);
        url
    }
}
fn get_delivery_signature(param_url_delivery: String) -> String {
    let mut hasher = Sha256::new();
    let to_sign = format!("{}{}", param_url_delivery, get_api_key_secret());
    println!("get delivery signature {}", &to_sign);
    hasher.update(to_sign.as_bytes());

    let hash = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());
    println!("get delivery hash {}", &hash);
    format!("s--{}--", hash.chars().take(8).collect::<String>())
}

fn get_upload_attributes(
    preset: String,
    folder: String,
    tags: Vec<String>,
    timestamp: i64,
    public_id: Option<String>,
) -> HashMap<String, String> {
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("upload_preset".to_string(), preset);
    attributes.insert("folder".to_string(), folder);
    attributes.insert("tags".to_string(), tags.join(","));
    attributes.insert("api_key".to_string(), get_api_key());
    attributes.insert("timestamp".to_string(), timestamp.to_string());

    if let Some(id) = public_id {
        attributes.insert("public_id".to_string(), id);
    }

    let signature = get_write_signature(Some(attributes.clone()));
    attributes.insert("signature".to_string(), signature);
    attributes
}

async fn upload_file_picture(
    file_bytes: Vec<u8>,
    filename: String,
    preset: String,
    folder: String,
    tags: Vec<String>,
) -> Result<CloudinaryUploadResponse, String> {
    let multipart = multipart::Part::bytes(file_bytes).file_name(filename);
    let mut form = multipart::Form::new().part("file", multipart);
    send_upload_picture(form, preset, folder, tags, None).await
}
pub async fn upload_url_picture(
    remmote_url: String,
    preset: String,
    folder: String,
    tags: Vec<String>,
) -> Result<CloudinaryUploadResponse, String> {
    let mut form = multipart::Form::new();
    form = form.text("file", remmote_url);
    send_upload_picture(form, preset, folder, tags, None).await
}

pub async fn migrate_picture_for_symbole(
    symbole_id: &str,
    remote_url: &str,
) -> Result<AssetUrl, String> {
    let symbole_tag = format!("symbole-{}", symbole_id);
    let tags = Vec::from([
        "migration".to_string(),
        "symbole".to_string(),
        symbole_tag.clone(),
    ]);
    //attributes.
    let folder = format!("{}/{}/{}", get_app_tag(), "symbole", symbole_tag);
    let result =
        upload_url_picture(remote_url.to_string(), "falidex".to_string(), folder, tags).await;
    match result {
        Ok(response) => update_cache_after_upload(response, symbole_id, false).await,
        Err(error) => Err(error),
    }
}

async fn send_upload_picture(
    mut form: multipart::Form,
    preset: String,
    folder: String,
    tags: Vec<String>,
    public_id: Option<String>,
) -> Result<CloudinaryUploadResponse, String> {
    let timestamp = Utc::now().timestamp();
    let attributes = get_upload_attributes(preset, folder, tags, timestamp, public_id);
    for (key, value) in &attributes {
        form = form.text(key.clone(), value.clone());
    }
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        get_cloud_name()
    );
    let client = Client::new();
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

fn gen_cloudinary_asset(
    asset_id: String,
    public_id: String,
    id: Option<String>,
) -> CloudinaryAssetWithId {
    CloudinaryAssetWithId {
        id: id.unwrap_or(Uuid::new_v4().to_string()),
        public_id: public_id,
        asset_id: asset_id,
    }
}

fn get_asset_url_from_local_id(local_id: String) -> AssetUrl {
    AssetUrl {
        id: local_id.clone(),
        url: format!("/resource/{}/800/800", local_id.clone()),
        thumbnail: format!("/resource/{}/100/100", local_id.clone()),
    }
}

async fn update_cache_after_upload(
    response: CloudinaryUploadResponse,
    symbole_id: &str,
    delete_cache_url: bool,
) -> Result<AssetUrl, String> {
    if let (Some(public_id), Some(asset_id)) =
        (response.public_id.clone(), response.asset_id.clone())
    {
        let asset_cache_id = FileCache::new(".app_temp/asset_cache_id");
        let asset_cache_asset_id = FileCache::new(".app_temp/asset_cache_asset_id");
        let asset = gen_cloudinary_asset(asset_id.clone(), public_id.clone(), None);
        let _ = asset_cache_id.set(&asset.id, &asset, 24 * 3600).await;
        let _ = asset_cache_asset_id
            .set(&asset.asset_id, &asset, 24 * 3600)
            .await;

        // Met à jour le cache du dossier concerné
        // Reset le cache du symbole (clé = symbole_id)

        if delete_cache_url {
            let cache = FileCache::new(".app_temp/urls");
            let _ = cache.delete(symbole_id).await;
        }
        Ok(get_asset_url_from_local_id(asset.id.clone()))
    } else {
        Err("Cloudinary: public_id ou asset_id manquant dans la réponse".to_string())
    }
}

pub fn clean_cache_expired() {
    let caches = Vec::<FileCache>::from([
        FileCache::new(".app_temp/asset_cache_id"),
        FileCache::new(".app_temp/asset_cache_asset_id"),
        FileCache::new(".app_temp/urls"),
        FileCache::new(".app_temp/delivery_url"),
        FileCache::new(".app_temp/pictures"),
    ]);
    for cache in caches {
        let _ = cache.clean_expired();
    }
}

fn get_api_key() -> String {
    env::get_cloudinary_api_key()
}

fn get_api_key_secret() -> String {
    env::get_cloudinary_api_secret_key()
}

fn get_cloud_name() -> String {
    env::get_cloudinary_cloud_name()
}

fn get_app_tag() -> String {
    env::get_cloudinary_main_folder()
}
