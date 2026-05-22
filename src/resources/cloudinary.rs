use crate::db::falidex::symbole;
use crate::env;
use axum::body::Bytes;
use base64::Engine;
use base64::engine::general_purpose;
use chrono::Utc;
use cloudinary::tags::{Tag, get_tags};
use cloudinary::upload::result::UploadResult;
use cloudinary::upload::{DeliveryType, OptionalParameters, ResourceTypes, Source, Upload};
use sha2::Sha256;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::format;
use std::time;

pub async fn upload_picture_for_symbole(
    data: &Bytes,
    content_type: &str,
    symbole_id: &str,
) -> Result<String, String> {
    let b64 = general_purpose::STANDARD.encode(data);
    let data_url = format!("data:{};base64,{}", content_type, b64);
    let tags = std::collections::HashSet::from([format!("symbole-{}", symbole_id)]);
    upload_data_url(data_url, tags).await // temp
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
            eprintln!(message);
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
fn get_signature_upload(atrribut_to_send: Option<HashMap<String, String>>) -> String {
    let not_allowed_keys = std::collections::HashSet::from(["file", "cloud_name", "api_key"]);
    let timestamp = Utc::now().timestamp();
    let attributes_string = atrribut_to_send
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
        attributes_string
        timestamp,
        get_api_key_secret()
    );
    let mut hasher = Sha256::new();
    hasher.update(to_serialize.as_bytes());
    hex::encode(hasher.finalize())
}
fn get_signature_delivery(id: String, attribut_to_send: Option<Vec<String>>) -> String {
    let attributes_string = attribut_to_send.unwrap_or_default().join(",");
    let to_sign = if attributes_string.is_empty() {
        format!("{}/{}", id, get_api_key_secret())
    } else {
        format!("{}/{}{}", attributes_string, id, get_api_key_secret())
    };
    let mut hasher = Sha256::new();
    hasher.update(to_sign.as_bytes());
    hex::encode(hasher.finalize())
}

fn get_options(tags: HashSet<String>) -> BTreeSet<OptionalParameters> {
    let mut optionTags = std::collections::HashSet::from([get_app_tag()]);
    optionTags.extend(tags);
    BTreeSet::from([
        OptionalParameters::ResourceType(ResourceTypes::Image),
        OptionalParameters::Type(DeliveryType::Private),
        OptionalParameters::Tags(optionTags),
        /*OptionalParameters::AssetFolder(get_app_tag()),
        OptionalParameters::Transformation(vec![Transformations::Crop(CropMode::Fill {
            width: 800,
            height: 800,
            gravity: None, // center par défaut
        })]),*/
    ])
}

/*
hurl = 'https://res.cloudinary.com/demo/image/authenticated/' + ([signature, to_sign]).join("/")
*/
