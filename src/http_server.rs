use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::db::DB;

pub fn build_router() -> Router {
    let r = Router::new()
        .route("/health", get(health))
        .route("/query-rooms", get(query_rooms))
        .route("/test-post", post(test_post));
    r
}

async fn health() -> impl IntoResponse {
    Json("Health")
}

async fn query_rooms() -> impl IntoResponse {
    let db = DB.lock().await;
    let v = json!(*db);
    Json(v)
}

async fn test_post(Json(body): Json<Value>) -> impl IntoResponse {
    println!("{:#?}", body);
    Json(body)
}
