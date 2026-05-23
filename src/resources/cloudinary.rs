use crate::env;
use chrono::Utc;
use cloudinary::tags::{Tag, get_tags};
use cloudinary::upload::result::UploadResult;
use cloudinary::upload::{DeliveryType, OptionalParameters, ResourceTypes, Source, Upload};
use reqwest::{Client, multipart};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet, hash_map};

pub async fn upload_picture_for_symbole(
    file_bytes: Vec<u8>,
    filename: String,
    symbole_id: &str,
) -> Result<String, String> {
    let symbole_tag = format!("symbole-{}", symbole_id);
    let tags = Vec::from([symbole_tag]);
    //attributes.
    let result = upload_picture(
        file_bytes,
        filename,
        "falidex".to_string(),
        format!("{}/{}", "falidex", "symbole_tag"),
        tags,
    )
    .await;
    match result {
        Ok(response) => Ok("ok".to_string()),
        Err(e) => Err(format!("Erreur HTTP: {}", e)),
    }
    //upload_data_url(data_url, tags).await // temp
}

pub async fn remove_picture() {}

/**
 * return cloudinary resources from given tags
 */
pub async fn get_asset_by_tag(tags: HashSet<String>) -> Result<Vec<Tag>, String> {
    let mut request_tags = std::collections::HashSet::from([get_app_tag()]);
    request_tags.extend(tags);
    let result_tags = get_tags(get_cloud_name().into(), "tag_name".into()).await;
    match result_tags {
        Ok(tag_list) => Ok(tag_list.resources),
        Err(error) => {
            let message = format!("Error getting picture: {}", error);
            eprintln!("{}", message);
            return Err(message);
        }
    }
}

async fn upload_data_url(data_url: String, tags: HashSet<String>) -> Result<String, String> {
    let upload = Upload::new(get_api_key(), get_cloud_name(), get_api_key_secret());
    let options = get_options(tags);
    let result = upload
        .image(Source::DataUrl(data_url), &options)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        UploadResult::Response(r) => {
            println!("success upload picture :url:{}", r.asset_id);
            Ok(r.asset_id)
        }
        UploadResult::ResponseWithImageMetadata(r) => {
            println!("success upload picture 2:url:{}", r.secure_url);
            Ok(r.asset_id)
        }
        UploadResult::Error(e) => {
            println!("errro upload picture {:?}", e);
            Err(format!("Cloudinary upload error: {:?}", e))
        }
    }
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
    let not_allowed_keys = std::collections::HashSet::from(["file", "cloud_name", "api_key"]);
    let timestamp = Utc::now().timestamp();
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
    let to_serialize = format!(
        "{}timestamp={}{}",
        attributes_string,
        timestamp,
        get_api_key_secret()
    );
    let mut hasher = Sha256::new();
    hasher.update(to_serialize.as_bytes());
    hex::encode(hasher.finalize())
}
fn get_delivery_url(id: String, attributs: Option<Vec<String>>) -> String {
    let param_url = get_delivery_param_url_(id.clone(), attributs.clone());
    let signature = get_delivery_signature(param_url.clone());
    format!(
        "https://res.cloudinary.com/{}/image/authenticated/{}/{}",
        get_cloud_name(),
        signature,
        param_url
    )
}

fn get_delivery_param_url_(id: String, attributs: Option<Vec<String>>) -> String {
    let attributes_string = attributs.unwrap_or_default().join(",");
    if attributes_string.is_empty() {
        format!("{}", id)
    } else {
        format!("{}/{}", attributes_string, id)
    }
}
fn get_delivery_signature(param_url_delivery: String) -> String {
    let mut hasher = Sha256::new();
    let to_sign = format!("{}{}", param_url_delivery, get_api_key_secret());
    hasher.update(to_sign.as_bytes());
    let hash = hex::encode(hasher.finalize());
    format!("s--{}--", hash.chars().take(8).collect::<String>())
}

fn get_options(tags: HashSet<String>) -> BTreeSet<OptionalParameters> {
    let mut option_tags = std::collections::HashSet::from([get_app_tag()]);
    option_tags.extend(tags);
    BTreeSet::from([
        OptionalParameters::ResourceType(ResourceTypes::Image),
        OptionalParameters::Type(DeliveryType::Private),
        OptionalParameters::Tags(option_tags),
        /*OptionalParameters::AssetFolder(get_app_tag()),
        OptionalParameters::Transformation(vec![Transformations::Crop(CropMode::Fill {
            width: 800,
            height: 800,
            gravity: None, // center par défaut
        })]),*/
    ])
}

async fn upload_picture(
    file_bytes: Vec<u8>,
    filename: String,
    preset: String,
    folder: String,
    tags: Vec<String>,
) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        get_cloud_name()
    );
    let client = Client::new();
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("preset".to_string(), preset);
    attributes.insert("folder".to_string(), folder);
    attributes.insert("tags".to_string(), tags.join(","));
    attributes.insert("api_key".to_string(), get_api_key());
    let signature = get_signature_upload(Some(attributes.clone()));
    attributes.insert("signature".to_string(), signature);
    let multipart = multipart::Part::bytes(file_bytes).file_name(filename);
    let mut form = multipart::Form::new().part("file", multipart);
    for (key, value) in &attributes {
        form = form.text(key.clone(), value.clone());
    }
    client.post(&url).form(&attributes).send().await
}
