use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // our router

    let w_api_key="51c6f97699546a4064ba2ef45de0aa14";
let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));

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
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}


async fn get_current_weather() -> Result<String, reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat=43.0&lon=6.6&appid={}","eb0b873a85379b2759eda56604289ce1");
    let response = reqwest::get(url).await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(body)
}