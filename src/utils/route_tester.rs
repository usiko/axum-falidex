use crate::env;

pub async fn testing_cors(origins: &[&str]) {
    let cors = env::get_allowed_origins();
    println!("\n=== Test CORS (CORS_ORIGIN={}) ===", cors);
    println!(
        "{:<35} {:<10} {:<35} {:<35} {}",
        "Origin", "Route", "Expected", "Actual", ""
    );
    println!("{}", "-".repeat(120));
    for origin in origins {
        request_token(origin, &cors).await;
        request_token_with_credentials(origin).await;
    }
    println!();
}

async fn request_token(origin: &str, cors_config: &str) {
    let port = env::get_port();
    let client = reqwest::Client::new();
    let result = client
        .request(
            reqwest::Method::OPTIONS,
            format!("http://localhost:{}/token", port),
        )
        .header("Origin", origin)
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "content-type")
        .send()
        .await;

    print_cors_result("/token (preflight)", origin, cors_config, result);
}

async fn request_token_with_credentials(origin: &str) {
    let port = env::get_port();
    let client = reqwest::Client::new();
    // Payload intentionnellement invalide : on teste le CORS, pas l'auth.
    // Le header ACAO doit être présent même sur une réponse 401.
    let result = client
        .post(format!("http://localhost:{}/token", port))
        .header("Origin", origin)
        .header("Content-Type", "application/json")
        .body(r#"{"role":"test","hash":"invalid","timestamp":0}"#)
        .send()
        .await;

    match result {
        Ok(resp) => {
            let acao = resp
                .headers()
                .get("access-control-allow-origin")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("<absent>");
            let acac = resp
                .headers()
                .get("access-control-allow-credentials")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("<absent>");
            // La combinaison * + true est invalide pour un navigateur
            let valid = !(acao == "*" && acac == "true");
            let marker = if valid {
                "✅"
            } else {
                "❌ ERREUR: * + credentials=true invalide"
            };
            println!(
                "{:<35} {:<25} ACAO={:<30} ACAC={} {}",
                origin, "/token (credentials)", acao, acac, marker
            );
        }
        Err(e) => println!(
            "{:<35} {:<25} ERREUR: {}",
            origin, "/token (credentials)", e
        ),
    }
}

fn print_cors_result(
    route: &str,
    origin: &str,
    cors_config: &str,
    result: Result<reqwest::Response, reqwest::Error>,
) {
    let expected = if cors_config == "*" {
        origin
    } else {
        let allowed: Vec<&str> = cors_config.split(',').map(|o| o.trim()).collect();
        if allowed.contains(&origin) {
            origin
        } else {
            "<absent>"
        }
    };

    match result {
        Ok(resp) => {
            let actual = resp
                .headers()
                .get("access-control-allow-origin")
                .map(|v| v.to_str().unwrap_or("<invalide>"))
                .unwrap_or("<absent>");
            let marker = if actual == expected { "✅" } else { "❌" };
            println!(
                "{:<35} {:<10} {:<35} {:<35} {}",
                origin, route, expected, actual, marker
            );
        }
        Err(e) => {
            println!(
                "{:<35} {:<10} {:<35} ERREUR: {}",
                origin, route, expected, e
            );
        }
    }
}
