use axum::{http, Json, extract};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

#[derive(Serialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    pub fn new(quote: String, book: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
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
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.quote, payload.book);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.inserted_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(quote))),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}