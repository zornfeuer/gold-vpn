use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use crate::{AppState, handlers};

pub async fn handler(
    State(state): State<AppState>,
    Path(platform): Path<String>,
) -> Json<serde_json::Value> {
    match handlers::subscribe::generate_config(&state.db_pool, &platform).await {
        Ok(config) => Json(json!(config)),
        Err(e) => {
            tracing::error!("Error generating config: {}", e);
            Json(json!({"error": e.to_string()}))
        }
    }
}
