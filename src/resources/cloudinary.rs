use cloudinary::upload::{OptionalParameters, Source, Upload};
use std::collections::BTreeSet;

pub async fn upload_picture(data: Bytes, content_type: &str) -> impl IntoResponse {
    let b64 = general_purpose::STANDARD.encode(&data);
    let data_url = format!("data:{};base64,{}", content_type, b64);
    println!("should upload {}", data_url);
    //uploadDataUrl(data_url).await;
}

pub async fn remove_picture() {}

async fn upload_data_url(data_url: String) {
    let options = BTreeSet::from([OptionalParameters::PublicId("1x1.png".to_string())]);
    let upload = Upload::new(
        "api_key".to_string(),
        "cloud_name".to_string(),
        "api_secret".to_string(),
    );
    let result = upload.image(Source::DataUrl(data_url), &options);
}
