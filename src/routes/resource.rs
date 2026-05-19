use crate::resources::{self, cloudinary::upload_picture};
use axum::{Router, extract::Multipart, routing::post};
use resources::cloudinary;

pub async fn upload_resource_picture(mut multipart: Multipart, Path(user_id): Path<String>) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data: bytes::Bytes = field.bytes().await.unwrap();
        upload_picture(data, &"".to_string()).await?;
        println!("Length of `{}` is {} bytes", name, data.len());
    }
}
