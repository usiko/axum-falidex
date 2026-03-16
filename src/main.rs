mod db;
mod routes;
mod state;

use axum::response::IntoResponse;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use routes::persistence::{get_persistence, set_persistence};
use state::{AppState, get_state};

#[tokio::main]
async fn main() {
    let app_state = get_state().await;
    let free = Router::new()
        .route("/", get(root))
        .route("/auth", post(auth))
        .with_state(app_state.clone());
    let protected = Router::new()
        .route("/persistence", get(get_persistence).post(set_persistence))
        .with_state(app_state);
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
