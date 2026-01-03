use anyhow::Result;
use serde_json::Value;
use reqwest::Client;
use uuid::Uuid;

/// MCP Adapter for communicating with sandbox-mcp
#[allow(dead_code)]
pub struct McpAdapter {
    client: Client,
    base_url: String,
}

#[allow(dead_code)]
impl McpAdapter {
    /// Create a new MCP adapter
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Submit a task to the sandbox
    pub async fn submit_task(&self, task: Value) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        
        tracing::info!("Submitting task {} to sandbox", task_id);
        
        let response = self
            .client
            .post(format!("{}/tasks", self.base_url))
            .json(&serde_json::json!({
                "id": task_id,
                "task": task
            }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(task_id)
        } else {
            anyhow::bail!("Failed to submit task: {}", response.status())
        }
    }

    /// Poll task status
    pub async fn poll_status(&self, task_id: &str) -> Result<Value> {
        tracing::debug!("Polling status for task {}", task_id);
        
        let response = self
            .client
            .get(format!("{}/tasks/{}/status", self.base_url, task_id))
            .send()
            .await?;

        if response.status().is_success() {
            let result = response.json::<Value>().await?;
            Ok(result)
        } else {
            anyhow::bail!("Failed to poll status: {}", response.status())
        }
    }

    /// Get task result
    pub async fn get_result(&self, task_id: &str) -> Result<Value> {
        tracing::debug!("Getting result for task {}", task_id);
        
        let response = self
            .client
            .get(format!("{}/tasks/{}/result", self.base_url, task_id))
            .send()
            .await?;

        if response.status().is_success() {
            let result = response.json::<Value>().await?;
            Ok(result)
        } else {
            anyhow::bail!("Failed to get result: {}", response.status())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_adapter_creation() {
        let adapter = McpAdapter::new("http://localhost:3000".to_string());
        assert_eq!(adapter.base_url, "http://localhost:3000");
    }
}
