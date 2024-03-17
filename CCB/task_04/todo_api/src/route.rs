use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::Mutex;

use crate::{
    handler::{
        delete_todo_item_path, get_health, get_index, get_todo_item_path, get_todo_items,
        patch_todo_item_path, post_todo_item,
    },
    model::AppState,
};

pub fn create_router() -> Router {
    let app_state = AppState {
        db: Arc::new(Mutex::new(Vec::new())),
    };

    Router::new()
        .route("/", get(get_index))
        .route("/api/health", get(get_health))
        .route("/api/todo-list", get(get_todo_items).post(post_todo_item))
        .route(
            "/api/todo-list/:id",
            get(get_todo_item_path)
                .patch(patch_todo_item_path)
                .delete(delete_todo_item_path),
        )
        .with_state(app_state)
}
