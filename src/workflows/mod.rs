/// Workflows module for handling workflow execution
///
/// This module provides a `Workflow` struct that allows you to define
/// and execute a series of steps asynchronously.
///
/// # Example
///
/// ```
/// let mut workflow = Workflow::new("example".to_string());
/// workflow.add_step("step1".to_string());
/// workflow.add_step("step2".to_string());
///
/// let result = workflow.execute().await;
/// assert!(result.is_ok());
/// ```
///
/// # Error Handling
///
/// The `execute` method returns a `Result<String, String>` where:
/// - `Ok(String)` contains the execution results
/// - `Err(String)` indicates an error (e.g., no steps to execute)
pub struct Workflow {
    pub name: String,
    pub steps: Vec<String>,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Workflow {
            name,
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: String) {
        self.steps.push(step);
    }

    pub async fn execute(&self) -> Result<String, String> {
        if self.steps.is_empty() {
            return Err("No steps to execute".to_string());
        }

        let mut results = Vec::new();
        results.push(format!("Workflow: {}", self.name));
        for step in &self.steps {
            results.push(format!("Executed: {}", step));
        }

        Ok(results.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_execution() {
        let mut workflow = Workflow::new("test_workflow".to_string());
        workflow.add_step("step1".to_string());
        workflow.add_step("step2".to_string());

        let result = workflow.execute().await;
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("step1"));
        assert!(output.contains("step2"));
    }

    #[tokio::test]
    async fn test_empty_workflow() {
        let workflow = Workflow::new("empty_workflow".to_string());
        let result = workflow.execute().await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No steps to execute");
    }
}
