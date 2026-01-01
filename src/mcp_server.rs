use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::agent::Agent;
use crate::claude::ClaudeAgent;

/// Application state
pub struct AppState {
    pub claude_agent: Arc<ClaudeAgent>,
}

/// Request for token counting
#[derive(Debug, Deserialize)]
pub struct TokenCountRequest {
    pub text: String,
}

/// Response for token counting
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCountResponse {
    pub count: usize,
}

/// Request for MCP routing
#[derive(Debug, Deserialize)]
pub struct McpRouteRequest {
    pub task: String,
    pub context: Option<Value>,
}

/// Response for MCP routing
#[derive(Debug, Serialize)]
pub struct McpRouteResponse {
    pub status: String,
    pub result: Value,
}

/// Configure routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/health", web::get().to(health_check))
        .route("/v1/messages/count_tokens", web::post().to(count_tokens))
        .route("/v1/mcp/route", web::post().to(mcp_route));
}

/// Index/welcome endpoint
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rust Web Framework!")
}

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "rust-web-router",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Count tokens in the provided text
async fn count_tokens(payload: web::Json<TokenCountRequest>) -> impl Responder {
    // Simple token counting (split by whitespace and punctuation)
    let count = payload.text
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .count();
    
    tracing::debug!("Counted {} tokens", count);
    
    HttpResponse::Ok().json(TokenCountResponse { count })
}

/// Route MCP requests to appropriate handlers
async fn mcp_route(
    state: web::Data<AppState>,
    payload: web::Json<McpRouteRequest>,
) -> impl Responder {
    tracing::info!("Routing MCP request for task: {}", payload.task);
    
    let input = json!({
        "task": payload.task,
        "context": payload.context
    });
    
    match state.claude_agent.handle(input).await {
        Ok(result) => HttpResponse::Ok().json(McpRouteResponse {
            status: "success".to_string(),
            result,
        }),
        Err(e) => {
            tracing::error!("Error handling MCP route: {}", e);
            HttpResponse::InternalServerError().json(McpRouteResponse {
                status: "error".to_string(),
                result: json!({
                    "error": e.to_string()
                }),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;
        
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;
        
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }
}
