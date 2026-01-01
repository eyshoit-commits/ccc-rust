mod agent;
mod claude;
mod workflows;
mod mcp_server;
mod mcp_adapter;

use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::claude::ClaudeAgent;
use crate::mcp_server::{AppState, configure_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_web_router=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Web Framework v{}", env!("CARGO_PKG_VERSION"));

    // Create application state
    let app_state = web::Data::new(AppState {
        claude_agent: Arc::new(ClaudeAgent::new()),
    });

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = format!("127.0.0.1:{}", port);
    tracing::info!("Server listening on {}", addr);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .configure(configure_routes)
    })
    .bind(&addr)?
    .run()
    .await
}
