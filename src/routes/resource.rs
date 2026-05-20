use crate::db::falidex;
use crate::resources::cloudinary::upload_picture;
use axum::Json;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn upload_resource_picture(
    Path(_user_id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut idPictures: Vec<String> = Vec::new();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();
        let data = field.bytes().await.unwrap();
        match upload_picture(&data, &content_type).await {
            Ok(id) => idPictures.push(id),
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    }
    symbole::add_picture(id)
    // ajouter les ids aux symbole concernés
    // return message success ou non
    //Json(urls).into_response()

    //ne pas update la bdd et faire plutot un dossie par symbole id!!
}
