use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "word_sorter=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    debug!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        // print message and break if instructed to do so
        match msg {
            Message::Text(t) => {
                debug!(">>> {who} sent str: {t:?}");
                if sender
                    .send(Message::Text(format!("{}", extract_words(t).join("\n"))))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Message::Binary(d) => {
                debug!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
            }
            Message::Close(c) => {
                if let Some(cf) = c {
                    debug!(
                        ">>> {} sent close with code {} and reason `{}`",
                        who, cf.code, cf.reason
                    );
                } else {
                    debug!(">>> {who} somehow sent close message without CloseFrame");
                }
                break;
            }
            Message::Pong(v) => {
                debug!(">>> {who} sent pong with {v:?}");
            }
            Message::Ping(v) => {
                debug!(">>> {who} sent ping with {v:?}");
            }
        }
    }

    // returning from the handler closes the websocket connection
    debug!("Websocket context {who} destroyed");
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../sorter.html"))
}

fn extract_words(text: String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut seen_words: std::collections::HashSet<String> = std::collections::HashSet::new();

    for word in text.split_whitespace() {
        let cleaned_word = word
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();
        if !cleaned_word.is_empty() && seen_words.insert(cleaned_word.clone()) {
            words.push(cleaned_word);
        }
    }

    words.sort();
    words
}
