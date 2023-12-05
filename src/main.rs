use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3000;
    let address = format!("0.0.0.0:{port}");

    let app = Router::new()
        .route("/", get(health));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn health() -> http::StatusCode {
    http::StatusCode::OK
}
