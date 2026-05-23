use serde::Deserialize;

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
