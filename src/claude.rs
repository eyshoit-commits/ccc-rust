use async_trait::async_trait;
use anyhow::Result;
use serde_json::{json, Value};
use crate::agent::Agent;

/// Claude Agent implementation for handling specific LLM workflow tasks
pub struct ClaudeAgent;

impl ClaudeAgent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Agent for ClaudeAgent {
    async fn handle(&self, input: Value) -> Result<Value> {
        // Extract task from input
        let task = input.get("task")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        
        tracing::info!("ClaudeAgent handling task: {}", task);
        
        // Process the task
        Ok(json!({
            "response": format!("Claude task '{}' completed", task),
            "status": "success",
            "agent": "claude"
        }))
    }
}

impl Default for ClaudeAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_claude_agent_handle() {
        let agent = ClaudeAgent::new();
        let input = json!({"task": "translate"});
        let result = agent.handle(input).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response["status"], "success");
        assert_eq!(response["agent"], "claude");
    }
}
