use axum::body::Bytes;
use base64::Engine;
use base64::engine::general_purpose;
use cloudinary::upload::result::UploadResult;
use cloudinary::upload::{DeliveryType, OptionalParameters, ResourceTypes, Source, Upload};
use std::collections::BTreeSet;

pub async fn upload_picture(data: &Bytes, content_type: &str) -> Result<String, String> {
    let b64 = general_purpose::STANDARD.encode(data);
    let data_url = format!("data:{};base64,{}", content_type, b64);
    //upload_data_url(data_url).await // temp
}

pub async fn remove_picture() {}

async fn upload_data_url(data_url: String) -> Result<String, String> {
    let upload = Upload::new(get_api_key(), get_cloud_name(), get_api_key_secret());
    let options = get_options();
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

fn get_options() -> BTreeSet<OptionalParameters> {
    BTreeSet::from([
        OptionalParameters::AssetFolder("falidex".to_string()),
        OptionalParameters::ResourceType(ResourceTypes::Image),
        OptionalParameters::Type(DeliveryType::Authenticated),
    ])
}
