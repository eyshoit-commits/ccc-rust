// Workflow engine module
use std::collections::HashMap;

pub struct WorkflowEngine {
    states: HashMap<String, String>,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub async fn execute(&self, agent: &str, state: &str, input: &str) -> Result<String, String> {
        // Simple state machine for demonstration
        match (agent, state) {
            (_, "init") => Ok("processing".to_string()),
            (_, "processing") => Ok("completed".to_string()),
            _ => Err("Invalid state".to_string()),
        }
    }
}

impl Default for WorkflowEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test uses tokio::test for proper async runtime support
    #[tokio::test]
    async fn test_workflow_execution() {
        let engine = WorkflowEngine::new();
        let agent = "test_agent";
        let input = "test_input";
        
        // Test initial state transition
        let next_state = engine.execute(&agent, "init", input).await;
        assert!(next_state.is_ok());
        assert_eq!(next_state.unwrap(), "processing");
        
        // Test processing state transition
        let next_state = engine.execute(&agent, "processing", input).await;
        assert!(next_state.is_ok());
        assert_eq!(next_state.unwrap(), "completed");
    }
}
