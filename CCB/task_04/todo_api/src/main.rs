mod handler;
mod model;
mod response;
mod route;

use crate::route::create_router;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, create_router()).await.unwrap();
}
