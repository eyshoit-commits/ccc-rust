use async_trait::async_trait;
use anyhow::Result;
use serde_json::Value;

/// Base trait that all agents must implement
#[async_trait]
pub trait Agent: Send + Sync {
    /// Handle incoming requests and return a response
    async fn handle(&self, input: Value) -> Result<Value>;
}
