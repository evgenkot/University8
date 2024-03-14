use axum::{routing::get, Router};

use crate::handler::{get_index, get_health};


pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/api/health", get(get_health))
}
