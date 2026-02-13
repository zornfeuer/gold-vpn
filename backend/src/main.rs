use tracing::{info, Level};
use axum::Router;
use tracing_subscriber;

mod config;
mod routes;
mod handlers;
mod utils;
mod db;

use db::Database;
use routes::{subscribe, health};

#[derive(Clone)]
struct AppState {
    db_pool: db::DbPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let database = Database::connect().await?;
    database.initialize().await?;
    
    let app_state = AppState {
        db_pool: database.pool.clone(),
    };
    
    let app = Router::new()
        .route("/subscribe/{platform}", axum::routing::get(subscribe))
        .route("/health", axum::routing::get(health))
        .with_state(app_state);
    
    let addr = "0.0.0.0:8080";
    info!("🚀 Panel listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
