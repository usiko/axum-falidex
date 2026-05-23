mod db;
mod encrypt;
mod env;
mod middleware;
mod model;
mod resources;
mod routes;
mod state;
mod token;

use crate::db::falidex::relations::fix_relation_id;
use crate::middleware::{verify_jwt_middleware, verify_token_middleware};
use crate::routes::falidex;
use crate::routes::falidex::symbole::{add_picture, redirect_picture};
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
use routes::persistence::{get_persistence, set_persistence};
use routes::users::{auth, get_user};
use state::get_state;
use tower_http::cors::{Any, CorsLayer};
#[tokio::main]
async fn main() {
    show_dev_ex_token("visitor");

    let app_state = get_state().await;
    match fix_relation_id(&app_state.db.db).await {
        Ok(msg) => println!("Fix relation IDs: {}", msg),
        Err(e) => eprintln!("Erreur lors du fix des IDs de relations: {}", e),
    }

    let cors = get_cors();
    let free = Router::new()
        .route("/token", post(verify_hash))
        .route("/collection/symboles/{id}/img/upload", post(add_picture))
        .route("/resource/{id}", get(redirect_picture))
        .with_state(app_state.clone())
        .layer(cors.clone());

    let token = Router::new()
        .route("/", get(root))
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
        );

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

    let protected = Router::new()
        .route("/persistence", get(get_persistence).post(set_persistence))
        .route("/user/", get(get_current_user))
        .merge(edit_route_falidex_link)
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
    let app = free.merge(token).merge(protected);
    // run our app with hyper, listening on the PORT environment variable (for Heroku) or 3000 by default
    let port = crate::env::get_port();
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

fn get_cors() -> CorsLayer {
    let allowed_origins = crate::env::get_allowed_origins();
    if allowed_origins == "*" {
        CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION, "X-Token".parse().unwrap()])
            .allow_origin(Any)
    } else {
        CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION, "X-Token".parse().unwrap()])
            .allow_origin(allowed_origins.parse::<axum::http::HeaderValue>().unwrap())
    }
}
