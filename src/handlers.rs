use axum::{http, Json, extract};
use axum::extract::{Path, State};
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
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(http::StatusCode, Json<Quote>), http::StatusCode> {
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
        Ok(_) => Ok((http::StatusCode::CREATED, Json(quote))),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn read_quotes(
    State(pool): extract::State<PgPool>,
) -> Result<(http::StatusCode, axum::Json<Vec<Quote>>), http::StatusCode> {
    let res = sqlx::query_as::<_,Quote>(r#"SELECT * FROM quotes"#)
    .fetch_all(&pool)
    .await;

    match res {
        Ok(quotes) => Ok((http::StatusCode::OK, axum::Json(quotes))),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn update_quote(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<CreateQuote>,
) -> http::StatusCode {
    let now = chrono::Utc::now();
    let res = sqlx::query(
        r#"
        UPDATE quotes
        SET book = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#
    )
    .bind(&payload.book)
    .bind(&payload.quote)
    .bind(now)
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            return http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}

pub async fn delete_quote(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> http::StatusCode {
    let res = sqlx::query(
        r#"
        DELETE FROM quotes
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            return http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}