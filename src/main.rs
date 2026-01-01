use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod workflows;
use workflows::Workflow;

#[derive(Deserialize, Serialize)]
struct TokenRequest {
    text: String,
}

#[derive(Serialize)]
struct TokenResponse {
    count: usize,
}

async fn count_tokens(req: web::Json<TokenRequest>) -> impl Responder {
    let count = req.text.split_whitespace().count();
    HttpResponse::Ok().json(TokenResponse { count })
}

async fn mcp_route() -> impl Responder {
    HttpResponse::Ok().body("MCP Route")
}

async fn execute_workflow() -> impl Responder {
    let mut workflow = Workflow::new("example".to_string());
    workflow.add_step("process".to_string());

    match workflow.execute().await {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rust Web Framework!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/v1/messages/count_tokens", web::post().to(count_tokens))
            .route("/v1/mcp/route", web::get().to(mcp_route))
            .route("/v1/workflow/execute", web::post().to(execute_workflow))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
