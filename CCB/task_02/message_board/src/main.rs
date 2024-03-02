use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures::{sink::SinkExt, stream::StreamExt};
use rusqlite::params;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing::{debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    connection: Mutex<rusqlite::Connection>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "message_board=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conn = match rusqlite::Connection::open("messages.db") {
        Ok(conn) => conn,
        Err(err) => {
            error!("Failed to open SQLite connection: {}", err);
            return;
        }
    };

    // Create messages table if not exists
    if let Err(err) = conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            timestamp TEXT,
            message TEXT
        )",
        [],
    ) {
        error!("Failed to create messages table: {}", err);
        return;
    }

    let app_state = Arc::new(AppState {
        connection: Mutex::new(conn),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
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
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    debug!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        // print message and break if instructed to do so
        match msg {
            Message::Text(t) => {
                debug!(">>> {who} sent str: {t:?}");
                if t.trim() == "LIST" {
                    match get_messages(&state.connection).await {
                        Ok(messages) => {
                            if sender.send(Message::Text(messages)).await.is_err() {
                                break;
                            }
                        }
                        Err(err) => {
                            if sender
                                .send(Message::Text(format!("Error: {}", err)))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                } else if t == "" {
                    if sender
                        .send(Message::Text("Bye!".to_string()))
                        .await
                        .is_err()
                    {
                        break;
                    };
                    break;
                } else {
                    match store_message(&state.connection, &t).await {
                        Ok(_) => {
                            let added_message = format!("Message added: \"{}\"", t);
                            if sender.send(Message::Text(added_message)).await.is_err() {
                                break;
                            }
                        }
                        Err(err) => {
                            if sender
                                .send(Message::Text(format!("Error: {}", err)))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
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
    Html(std::include_str!("../index.html"))
}

async fn store_message(conn: &Mutex<rusqlite::Connection>, message: &str) -> Result<(), String> {
    let timestamp = chrono::Local::now().to_rfc3339();

    if let Ok(conn) = conn.lock() {
        if let Err(err) = conn.execute(
            "INSERT INTO messages (timestamp, message) VALUES (?, ?)",
            params![timestamp, message],
        ) {
            return Err(format!("Failed to store message in the database: {}", err));
        }
        Ok(())
    } else {
        Err(String::from("Failed to acquire database connection lock"))
    }
}

async fn get_messages(conn: &Mutex<rusqlite::Connection>) -> Result<String, String> {
    if let Ok(conn) = conn.lock() {
        if let Ok(mut stmt) = conn.prepare("SELECT message FROM messages") {
            let messages: Result<Vec<String>, _> = stmt
                .query_map(params![], |row| row.get(0))
                .unwrap()
                .collect();

            if let Ok(messages) = messages {
                Ok(messages.join("; "))
            } else {
                Err(String::from(
                    "Failed to retrieve messages from the database",
                ))
            }
        } else {
            Err(String::from("Failed to prepare SQL statement"))
        }
    } else {
        Err(String::from("Failed to acquire database connection lock"))
    }
}
