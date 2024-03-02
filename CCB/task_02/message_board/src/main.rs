use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    routing, Router,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "message_board=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", routing::get(handler));
    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> response::Html<&'static str> {
    response::Html("<h1>Hello world!</h1>")
}

async fn handler_404() -> impl IntoResponse {
    tracing::debug!("404");
    (StatusCode::NOT_FOUND, "There is nothing to hide...")
}
