//! Example of using the Rust Web Router library programmatically
//!
//! This example demonstrates how to:
//! 1. Create agents
//! 2. Use the workflow engine
//! 3. Process tasks with different agents

use serde_json::json;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Rust Web Router Example");
    println!("========================\n");

    // Example 1: Using ClaudeAgent directly
    println!("Example 1: Direct Agent Usage");
    use rust_web_router::{agent::Agent, claude::ClaudeAgent};
    
    let claude = ClaudeAgent::new();
    let result = claude
        .handle(json!({
            "task": "translate",
            "text": "Hello, world!"
        }))
        .await?;
    println!("Result: {}\n", serde_json::to_string_pretty(&result)?);

    // Example 2: Using Workflow Engine
    println!("Example 2: Workflow Engine");
    use rust_web_router::workflows::WorkflowEngine;
    
    let engine = WorkflowEngine::new();
    let next_state = engine
        .execute(
            &claude,
            "init",
            json!({
                "task": "analyze",
                "data": "Sample data"
            }),
        )
        .await?;
    println!("Next state: {}\n", next_state);

    // Example 3: Processing multiple tasks
    println!("Example 3: Batch Processing");
    let tasks = vec![
        ("summarize", "Long text to summarize..."),
        ("classify", "Text to classify..."),
        ("extract", "Text to extract from..."),
    ];

    for (task, text) in tasks {
        let result = claude
            .handle(json!({
                "task": task,
                "text": text
            }))
            .await?;
        println!(
            "Task '{}' result: {}",
            task,
            serde_json::to_string(&result)?
        );
    }

    println!("\nExample completed successfully!");
    Ok(())
}
