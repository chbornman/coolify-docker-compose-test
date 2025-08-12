use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Serialize, Deserialize)]
struct Counter {
    value: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Ensure the data directory exists
    std::fs::create_dir_all("/data").ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:/data/counter.db?mode=rwc".to_string());
    
    info!("Connecting to database: {}", database_url);
    
    // Connect with create if not exists mode
    let pool = SqlitePool::connect(&database_url).await?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS counter (
            id INTEGER PRIMARY KEY,
            value INTEGER NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;
    
    sqlx::query("INSERT OR IGNORE INTO counter (id, value) VALUES (1, 0)")
        .execute(&pool)
        .await?;
    
    let state = Arc::new(AppState { db: pool });
    
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .route("/api/counter/reset", post(reset_counter))
        .route("/api/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;
    
    info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn get_counter(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Counter>, StatusCode> {
    let row = sqlx::query("SELECT value FROM counter WHERE id = 1")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let value: i64 = row.get("value");
    
    Ok(Json(Counter { value }))
}

async fn increment_counter(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Counter>, StatusCode> {
    sqlx::query("UPDATE counter SET value = value + 1 WHERE id = 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    get_counter(State(state)).await
}

async fn decrement_counter(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Counter>, StatusCode> {
    sqlx::query("UPDATE counter SET value = value - 1 WHERE id = 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    get_counter(State(state)).await
}

async fn reset_counter(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Counter>, StatusCode> {
    sqlx::query("UPDATE counter SET value = 0 WHERE id = 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(Counter { value: 0 }))
}