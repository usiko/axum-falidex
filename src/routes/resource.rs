use crate::resources::cloudinary::upload_picture;
use axum::extract::Multipart;
use axum::extract::Path;

pub async fn upload_resource_picture(Path(user_id): Path<String>, mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        upload_picture(&data, &"".to_string()).await;
        println!("Length of `{}` is {} bytes", name, data.len());
    }
}
