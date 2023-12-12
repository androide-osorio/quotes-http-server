use axum::{http, Json, extract};
use sqlx::PgPool;

#[derive(Debug, serde::Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> http::StatusCode {
    println!("Received request to create quote: {:?}", payload);
    http::StatusCode::CREATED
}