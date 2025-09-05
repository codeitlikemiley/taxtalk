// Axum server for Crux + WASM Plugin Architecture
// Provides HTTP/SSE/WebSocket bridge for chat interface

use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
// use core::PluginSystem; // TODO: Implement when PluginSystem is ready
use std::net::SocketAddr;
// use std::sync::Arc; // TODO: Uncomment when PluginSystem is ready
// use tokio::sync::Mutex; // TODO: Uncomment when PluginSystem is ready
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: Initialize plugin system when implemented
    // let plugin_system = Arc::new(Mutex::new(PluginSystem::new()));
    // TODO: Load plugins from server/plugins/ directory

    // Create broadcast channel for chat messages
    let (tx, _rx) = broadcast::channel(100);

    // Build application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .route("/health", get(health_check))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .with_state(AppState {
            // plugin_system, // TODO: Add when PluginSystem is ready
            chat_tx: tx,
        });

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    // plugin_system: Arc<Mutex<PluginSystem>>, // TODO: Add when PluginSystem is ready
    chat_tx: broadcast::Sender<String>,
}

async fn root() -> &'static str {
    "Crux Plugin Server - Chat Interface"
}

async fn health_check() -> &'static str {
    "OK"
}

fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .route("/health", get(health_check))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.chat_tx.subscribe();

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // TODO: Process through plugin system when Send constraints are resolved
                        // For now, just echo the message
                        let response_text = format!("Echo: {}", text);
                        let _ = state.chat_tx.send(response_text.clone());
                        let _ = socket.send(Message::Text(response_text)).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        return;
                    }
                    _ => {}
                }
            }

            // Handle broadcast messages
            msg = rx.recv() => {
                match msg {
                    Ok(text) => {
                        let _ = socket.send(Message::Text(text)).await;
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    fn create_test_app() -> Router {
        let (tx, _rx) = broadcast::channel(100);
        create_router().with_state(AppState { chat_tx: tx })
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_test_app();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;

        response.assert_status_ok();
        response.assert_text("OK");
    }

    #[tokio::test]
    async fn test_chat_endpoint() {
        let app = create_test_app();
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/chat")
            .json(&serde_json::json!({"message": "test"}))
            .await;

        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_root() {
        let app = Router::new().route("/", get(root));
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;
        response.assert_text("Crux Plugin Server - Chat Interface");
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new().route("/health", get(health_check));
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;
        response.assert_text("OK");
    }
}
