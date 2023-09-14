use axum::{Router, response::IntoResponse, Json, routing::get};

pub fn build_router() -> Router {
    let r = Router::new()
        .route("/health", get(health));

    r
}

async fn health() -> impl IntoResponse {
    Json("Health")
}