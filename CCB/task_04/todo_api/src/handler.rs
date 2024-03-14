use axum::{response, Json};

pub async fn get_health() -> impl response::IntoResponse {
    let json_response = serde_json::json!({
        "message": "all good",
        "status": "ok"
    });
    Json(json_response)
}

pub async fn get_index() -> response::Html<&'static str> {
    response::Html(std::include_str!("../index.html"))
}