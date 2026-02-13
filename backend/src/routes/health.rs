use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::json;
use tracing::error;
use crate::AppState;

/// Health check endpoint
/// 
/// Проверяет:
/// - Работоспособность приложения
/// - Подключение к базе данных
pub async fn handler(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    // Проверяем подключение к базе данных
    match sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "status": "healthy",
                "database": "connected",
                "service": "panel-backend",
                "version": env!("CARGO_PKG_VERSION")
            })),
        ),
        Err(e) => {
            error!("Health check failed: database connection error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "unhealthy",
                    "database": "disconnected",
                    "error": e.to_string()
                })),
            )
        }
    }
}
