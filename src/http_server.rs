use axum::{Router, response::IntoResponse, Json, routing::get};

use crate::db::DB;

pub fn build_router() -> Router {
    let r = Router::new()
        .route("/health", get(health))
        .route("/query-rooms", get(query_rooms));
    r
}

async fn health() -> impl IntoResponse {
    Json("Health")
}

async fn query_rooms() -> impl IntoResponse {
    let db = DB.lock().await;
    let v = serde_json::json!(*db);
    Json(v)
}