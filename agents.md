# Agent Specification for LLMs

This document explains the modular architecture for implementing agents in the Rust-based LLMs project.

## Module Explanation

### Agent Trait (`agent.rs`)
Defines a basic structure that every agent must implement:
```rust
#[async_trait]
pub trait Agent {
    async fn handle(&self, input: serde_json::Value) -> anyhow::Result<serde_json::Value>;
}
```
By adhering to this trait, agents are guaranteed to provide a `handle` implementation for processing tasks.

### Claude Agent (`claude.rs`)
Implements the `Agent` trait for handling specific tasks in the Claude LLM workflow.
```rust
pub struct ClaudeAgent;

#[async_trait]
impl Agent for ClaudeAgent {
    async fn handle(&self, input: Value) -> Result<Value> {
        // Implement logic for handling requests.
        Ok(json!({"response": "Claude task completed"}))
    }
}
```

### Workflow Integration (`workflows/mod.rs`)
Agents are orchestrated in workflows defined as Finite State Machines (FSMs) for better control over app logic. For example:
```rust
pub struct WorkflowEngine;
impl WorkflowEngine {
    pub fn execute(&self, agent: &dyn Agent, state: &str) -> Result<String> {
        // FSM handles state transitions
    }
}
```

## Example Usage

Here’s how to use the `ClaudeAgent`:
```rust
let agent = ClaudeAgent;
let result = agent.handle(json!({ "task": "translate" })).await;
println!("{:?}", result);
```