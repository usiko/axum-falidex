mod db;
mod encrypt;
mod middleware;
mod model;
mod routes;
mod state;
mod token;

use crate::db::falidex::fix_link_relation_id;
use crate::middleware::verify_token_middleware;
use crate::routes::falidex;
use crate::routes::{token::verify_hash, users::get_current_user};
use crate::token::show_dev_ex_token;
use axum::http::{
    Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};
use axum_jwt::layer;
use routes::persistence::{get_persistence, set_persistence};
use routes::users::{auth, get_user};
use state::get_state;
use tower_http::cors::{Any, CorsLayer};
#[tokio::main]
async fn main() {
    show_dev_ex_token("visitor");

    let app_state = get_state().await;
    match fix_link_relation_id(&app_state.db.db).await {
        Ok(msg) => println!("Fix relation IDs: {}", msg),
        Err(e) => eprintln!("Erreur lors du fix des IDs de relations: {}", e),
    }
    let jwt_decoder = app_state.jwt_decoder.clone();
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, "X-Token".parse().unwrap()])
        // allow requests from any origin
        .allow_origin(Any);
    let free = Router::new()
        .route("/token", post(verify_hash))
        .with_state(app_state.clone())
        .layer(cors.clone());
    let token = Router::new()
        .route("/", get(root))
        .route("/auth", post(auth))
        .route("/user/id/{user_id}", get(get_user))
        .route("/collection/circulaires", get(falidex::circulaire::get))
        .route(
            "/collection/circulaires-colors",
            get(falidex::circulaire_color::get),
        )
        .route("/collection/colors", get(falidex::color::get))
        .route("/collection/filieres", get(falidex::filiere::get))
        .route("/collection/placements", get(falidex::placement::get))
        .route("/collection/positions", get(falidex::position::get))
        .route(
            "/collection/significations",
            get(falidex::signification::get),
        )
        .route("/collection/symboles", get(falidex::symbole::get))
        .route(
            "/collection/symbole-accessoires",
            get(falidex::symbole_accessoire::get),
        )
        .route("/collection/symboles-sens", get(falidex::symbole_sens::get))
        .route("/collection/links", get(falidex::link::get))
        .route("/collection/link/{link_id}", get(falidex::link::get_item))
        .layer(from_fn_with_state(
            app_state.clone(),
            verify_token_middleware,
        ))
        .with_state(app_state.clone())
        .layer(cors.clone());
    let protected = Router::new()
        .route("/persistence", get(get_persistence).post(set_persistence))
        .route("/user/", get(get_current_user))
        .route(
            "/collection/link/{link_id}",
            put(falidex::link::update_item).delete(falidex::link::delete_item),
        )
        .route(
            "/collection/link/{link_id}/update",
            post(falidex::link::update_item_relation),
        )
        .route(
            "/collection/link/{link_id}/create",
            post(falidex::link::create_item_relation),
        )
        .route(
            "/collection/link/{link_id}/relation/{relation_id}",
            delete(falidex::link::delete_item_relation),
        )
        .layer(from_fn_with_state(
            app_state.clone(),
            verify_token_middleware,
        ))
        .with_state(app_state)
        .layer(layer(jwt_decoder))
        .layer(cors);
    let app = free.merge(token).merge(protected);
    // run our app with hyper, listening on the PORT environment variable (for Heroku) or 3000 by default
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
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
