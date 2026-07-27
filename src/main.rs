mod cache;
mod db;
mod encrypt;
mod env;
mod middleware;
mod migrate;
mod model;
mod resources;
mod routes;
mod security_utils;
mod state;
mod utils;
use std::time::Duration;

use crate::db::falidex::relations::fix_relation_id;
use crate::middleware::{verify_cookie_middleware, verify_jwt_middleware, verify_token_middleware};
use crate::migrate::migrate_symboles_imgs;
use crate::resources::cloudinary;
use crate::routes::falidex::symbole::{add_picture, get_picture};
use crate::routes::{falidex, resource};
use crate::routes::{token::verify_hash, users::get_current_user};
use crate::state::AppState;
use axum::http::{
    Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};
use dotenvy::dotenv;
use routes::persistence::{get_persistence, set_persistence};
use routes::users::{auth, get_user};
use state::get_state;
use tokio::time::interval;
use tower_http::cors::{AllowOrigin, CorsLayer};
#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_state = get_state().await;
    let fix_relation_state = app_state.clone();
    let webserver_state = app_state.clone();
    let migration_state = app_state.clone();
    tokio::spawn(async move {
        let is_activated = env::is_fix_relation_id_activated();
        if is_activated {
            match fix_relation_id(&fix_relation_state.db.db).await {
                Ok(msg) => println!("Fix relation IDs: {}", msg),
                Err(e) => eprintln!("Erreur lors du fix des IDs de relations: {}", e),
            }
        }
    });
    tokio::spawn(async move {
        let is_activated = env::is_migration_img_activated();
        if is_activated {
            migrate_symboles_imgs(&migration_state).await;
        }
    });
    tokio::spawn(async move { clean_cache_expired().await });
    if let Some(test_origins) = env::get_cors_test_origins() {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            let origins: Vec<&str> = test_origins.iter().map(|s| s.as_str()).collect();
            utils::route_tester::testing_cors(&origins).await;
        });
    }
    init_webserver(webserver_state).await
}

async fn init_webserver(app_state: AppState) {
    let cors = get_cors();
    let free = Router::new()
        .route("/token", post(verify_hash))
        .with_state(app_state.clone())
        .layer(cors.clone());
    let cookie_protected = Router::new()
        .route("/resource/{id}/{height}/{width}", get(get_picture))
        .with_state(app_state.clone())
        .layer(from_fn_with_state(
            app_state.clone(),
            verify_cookie_middleware,
        ))
        .layer(cors.clone());

    let token = Router::new()
        .route("/auth", post(auth))
        .route("/user/id/{user_id}", get(get_user))
        .route("/collection/circulaires", get(falidex::circulaire::get))
        .route(
            "/collection/circulaires/occurence/{id}",
            get(falidex::circulaire::get_occurences),
        )
        .route(
            "/collection/circulaires-colors",
            get(falidex::circulaire_color::get),
        )
        .route(
            "/collection/circulaires-colors/occurence/{id}",
            get(falidex::circulaire_color::get_occurences),
        )
        .route("/collection/colors", get(falidex::color::get))
        .route(
            "/collection/colors/occurence/{id}",
            get(falidex::color::get_occurences),
        )
        .route("/collection/filieres", get(falidex::filiere::get))
        .route(
            "/collection/filieres/occurence/{id}",
            get(falidex::filiere::get_occurences),
        )
        .route("/collection/placements", get(falidex::placement::get))
        .route(
            "/collection/placements/occurence/{id}",
            get(falidex::placement::get_occurences),
        )
        .route("/collection/positions", get(falidex::position::get))
        .route(
            "/collection/positions/occurence/{id}",
            get(falidex::position::get_occurences),
        )
        .route(
            "/collection/significations",
            get(falidex::signification::get),
        )
        .route(
            "/collection/significations/occurence/{id}",
            get(falidex::signification::get_occurences),
        )
        .route("/collection/symboles", get(falidex::symbole::get))
        .route(
            "/collection/symboles/occurence/{id}",
            get(falidex::symbole::get_occurences),
        )
        .route(
            "/collection/symbole-accessoires",
            get(falidex::symbole_accessoire::get),
        )
        .route(
            "/collection/symbole-accessoires/occurence/{id}",
            get(falidex::symbole_accessoire::get_occurences),
        )
        .route("/collection/symboles-sens", get(falidex::symbole_sens::get))
        .route(
            "/collection/symboles-sens/occurence/{id}",
            get(falidex::symbole_sens::get_occurences),
        )
        .route("/collection/links", get(falidex::link::get))
        .route("/collection/link/{link_id}", get(falidex::link::get_item))
        .route("/collection/import-log", get(falidex::import_log::get))
        .route(
            "/collection/import-log/{id}",
            get(falidex::import_log::get_item),
        )
        .layer(from_fn_with_state(
            app_state.clone(),
            verify_token_middleware,
        ))
        .with_state(app_state.clone())
        .layer(cors.clone());

    let edit_route_falidex_link = Router::new()
        .route(
            "/collection/link",
            post(falidex::link::create_item).put(falidex::link::update_item),
        )
        .route(
            "/collection/link/{link_id}",
            delete(falidex::link::delete_item),
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
            "/collection/link/{link_id}/relation-item/{relation_id}",
            delete(falidex::link::delete_item_relation),
        );

    let edit_route_falidex_circulaire = Router::new()
        .route("/collection/circulaires", post(falidex::circulaire::create))
        .route(
            "/collection/circulaires/{id}",
            put(falidex::circulaire::update).delete(falidex::circulaire::delete),
        );

    let edit_route_falidex_color = Router::new()
        .route("/collection/colors", post(falidex::color::create))
        .route(
            "/collection/colors/{id}",
            put(falidex::color::update).delete(falidex::color::delete),
        );

    let edit_route_falidex_filiere = Router::new()
        .route("/collection/filieres", post(falidex::filiere::create))
        .route(
            "/collection/filieres/{id}",
            put(falidex::filiere::update).delete(falidex::filiere::delete),
        );

    let edit_route_falidex_placement = Router::new()
        .route("/collection/placements", post(falidex::placement::create))
        .route(
            "/collection/placements/{id}",
            put(falidex::placement::update).delete(falidex::placement::delete),
        );

    let edit_route_falidex_position = Router::new()
        .route("/collection/positions", post(falidex::position::create))
        .route(
            "/collection/positions/{id}",
            put(falidex::position::update).delete(falidex::position::delete),
        );

    let edit_route_falidex_signification = Router::new()
        .route(
            "/collection/significations",
            post(falidex::signification::create),
        )
        .route(
            "/collection/significations/{id}",
            put(falidex::signification::update).delete(falidex::signification::delete),
        );

    let edit_route_falidex_symbole = Router::new()
        .route("/collection/symboles", post(falidex::symbole::create))
        .route(
            "/collection/symboles/{id}",
            put(falidex::symbole::update).delete(falidex::symbole::delete),
        )
        .route(
            "/collection/symboles/{id}/resource/upload",
            post(add_picture),
        )
        .route("/resource/remove/{id}", delete(resource::remove));

    let edit_route_falidex_symbole_accessoire = Router::new()
        .route(
            "/collection/symbole-accessoires",
            post(falidex::symbole_accessoire::create),
        )
        .route(
            "/collection/symbole-accessoires/{id}",
            put(falidex::symbole_accessoire::update).delete(falidex::symbole_accessoire::delete),
        );

    let edit_route_falidex_symbole_sens = Router::new()
        .route(
            "/collection/symboles-sens",
            post(falidex::symbole_sens::create),
        )
        .route(
            "/collection/symboles-sens/{id}",
            put(falidex::symbole_sens::update).delete(falidex::symbole_sens::delete),
        );

    let edit_route_falidex_circulaire_color = Router::new()
        .route(
            "/collection/circulaires-colors",
            post(falidex::circulaire_color::create),
        )
        .route(
            "/collection/circulaires-colors/{id}",
            put(falidex::circulaire_color::update).delete(falidex::circulaire_color::delete),
        );

    let edit_route_falidex_import = Router::new().route("/collection/import", post(falidex::import::create));

    let protected = Router::new()
        .route("/persistence", get(get_persistence).post(set_persistence))
        .route("/user/", get(get_current_user))
        .merge(edit_route_falidex_link)
        .merge(edit_route_falidex_import)
        .merge(edit_route_falidex_circulaire)
        .merge(edit_route_falidex_color)
        .merge(edit_route_falidex_filiere)
        .merge(edit_route_falidex_placement)
        .merge(edit_route_falidex_position)
        .merge(edit_route_falidex_signification)
        .merge(edit_route_falidex_symbole)
        .merge(edit_route_falidex_symbole_accessoire)
        .merge(edit_route_falidex_symbole_sens)
        .merge(edit_route_falidex_circulaire_color)
        .layer(from_fn_with_state(
            app_state.clone(),
            verify_token_middleware,
        ))
        .with_state(app_state.clone())
        .layer(from_fn_with_state(app_state, verify_jwt_middleware))
        .layer(cors);
    let app = free.merge(cookie_protected).merge(token).merge(protected);
    // run our app with hyper, listening on the PORT environment variable (for Heroku) or 3000 by default
    let port = crate::env::get_port();
    let addr = format!("0.0.0.0:{}", port);
    println!(
        "Server v{} listening on {}",
        env!("CARGO_PKG_VERSION"),
        addr
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn clean_cache_expired() {
    let mut interval = interval(Duration::from_mins(15));
    cloudinary::clean_cache_expired();
    loop {
        interval.tick().await;
        cloudinary::clean_cache_expired();
    }
}

fn get_cors() -> CorsLayer {
    let allowed_origins = crate::env::get_allowed_origins();
    let mut cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, "X-Token".parse().unwrap()])
        .allow_credentials(true);

    if allowed_origins == "*" {
        // Any + allow_credentials(true) est interdit par la spec HTTP ;
        // mirror_request renvoie l'origine du requêteur, ce qui autorise tout
        // en restant compatible avec les credentials.
        println!("cors allowed {:?}", AllowOrigin::mirror_request());
        cors = cors.allow_origin(AllowOrigin::mirror_request())
    } else {
        let origins: Vec<axum::http::HeaderValue> = allowed_origins
            .split(',')
            .map(|o| o.trim().parse::<axum::http::HeaderValue>().unwrap())
            .collect();
        println!("cors allowed {:?}", origins);
        cors = cors.allow_origin(AllowOrigin::list(origins))
    }
    cors
}
