use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CloudinaryUploadResponse {
    pub asset_id: Option<String>,
    pub public_id: Option<String>,
    pub version: Option<u64>,
    pub signature: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,
    pub resource_type: Option<String>,
    pub created_at: Option<String>,
    pub tags: Option<Vec<String>>,
    pub bytes: Option<u64>,
    pub r#type: Option<String>,
    pub etag: Option<String>,
    pub placeholder: Option<bool>,
    pub url: Option<String>,
    pub secure_url: Option<String>,
    pub folder: Option<String>,
    pub original_filename: Option<String>,
    pub error: Option<CloudinaryError>,
}

#[derive(Debug, Deserialize)]
pub struct CloudinaryError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetUrl {
    pub id: String,
    pub url: String,
    pub thumbnail: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteParams {
    pub asset_id: String,
    pub signature: String,
    pub api_key: String,
    pub timestamp: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudinaryAsset {
    pub public_id: String,
    pub asset_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudinaryAssetWithId {
    pub id: String,        // uuid v4
    pub public_id: String, // cloudinary public_id
    pub asset_id: String,  // cloudinary asset_id
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudinarySearchResponse {
    pub resources: Vec<CloudinaryAsset>,
}
