mod db;
use axum::response::IntoResponse;
use axum::{
    Json, Router,
    routing::{get, post},
};
use db::mongo::MongoDB;

#[tokio::main]
async fn main() {
    let free = Router::new()
        .route("/", get(root))
        .route("/auth", post(auth));
    let protected = Router::new().route("/persistence", get(get_persistence).post(set_persistence));
    let app = free.merge(protected);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    let base_message = "this is a rust web server!";

    let weather = match get_current_weather().await {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Erreur lors de la récupération de la météo : {}", e);
            return format!("{} Impossible de récupérer la météo.", base_message);
        }
    };

    // weather est déjà une String, pas besoin de unwrap
    format!("{}\n{}", base_message, weather)
}
async fn auth(val: String) {}
async fn get_persistence() -> impl IntoResponse {
    let db = MongoDB::new().await.expect("Failed to connect");
    let result = db
        .get_persistence("test".to_string())
        .await
        .expect("Failed to get");
    result
}
async fn set_persistence(body: String) -> impl IntoResponse {
    let db = MongoDB::new().await.expect("Failed to connect");
    let result = db
        .set_persistence("test".to_string(), body, true)
        .await
        .expect("Failed to get");
    result
}

async fn get_current_weather() -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat=43.0&lon=6.6&appid={}",
        "eb0b873a85379b2759eda56604289ce1"
    );
    let response = reqwest::get(url).await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(body)
}
