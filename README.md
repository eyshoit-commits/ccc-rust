# CCC Rust MCP Server

[![Rust CI](https://github.com/eyshoit-commits/ccc-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/eyshoit-commits/ccc-rust/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Cloud Code Router and Model Context Protocol (MCP) server implementation in Rust, providing efficient agent orchestration and task routing capabilities.

## Features

- 🚀 **High-Performance MCP Server**: Built with Axum for efficient HTTP handling
- 🤖 **Agent Framework**: Modular agent architecture with trait-based design
- 🔄 **Workflow Engine**: FSM-based workflow orchestration
- 📊 **Token Counting**: Built-in token analysis for LLM operations
- 🐳 **Docker Support**: Containerized deployment ready
- 🧪 **Comprehensive Testing**: Full test coverage with integration tests
- 🔧 **Developer-Friendly**: VSCode devcontainer support with Rust tooling

## Architecture

The project follows a modular architecture:

```
ccc-rust-mcp/
├── src/
│   ├── main.rs           # Server entry point
│   ├── agent.rs          # Agent trait definition
│   ├── claude.rs         # Claude agent implementation
│   ├── mcp_server.rs     # HTTP server and routing
│   ├── mcp_adapter.rs    # Sandbox MCP communication
│   └── workflows/        # Workflow orchestration
│       └── mod.rs
├── Cargo.toml            # Project manifest
├── Dockerfile            # Container image
├── docker-compose.yml    # Service composition
└── Makefile             # Build automation
```

## Quick Start

### Prerequisites

- Rust 1.83+ (install from [rustup.rs](https://rustup.rs))
- Docker (optional, for containerized deployment)
- Make (optional, for convenience commands)

### Installation

```bash
# Clone the repository
git clone https://github.com/eyshoit-commits/ccc-rust.git
cd ccc-rust

# Install dependencies
cargo fetch

# Build the project
cargo build
```

### Running the Server

```bash
# Run in development mode
cargo run

# Or use Make
make run

# The server will start on http://localhost:3000
```

### Using Docker

```bash
# Build the Docker image
docker build -t ccc-rust-mcp:latest .

# Run the container
docker run -p 3000:3000 ccc-rust-mcp:latest

# Or use docker-compose
docker-compose up -d
```

## API Endpoints

### Health Check
```bash
GET /health
```

Response:
```json
{
  "status": "healthy",
  "service": "ccc-rust-mcp",
  "version": "0.1.0"
}
```

### Count Tokens
```bash
POST /v1/messages/count_tokens
Content-Type: application/json

{
  "text": "Your text to count tokens"
}
```

Response:
```json
{
  "count": 5
}
```

### Route MCP Request
```bash
POST /v1/mcp/route
Content-Type: application/json

{
  "task": "translate",
  "context": {
    "additional": "data"
  }
}
```

Response:
```json
{
  "status": "success",
  "result": {
    "response": "Claude task 'translate' completed",
    "status": "success",
    "agent": "claude"
  }
}
```

## Development

### Prerequisites

The project includes a `.devcontainer` configuration for VSCode with all necessary tools pre-configured:
- Rust toolchain with rust-analyzer
- Docker support
- GitHub CLI
- Node.js (for sandbox-mcp integration)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Using Make
make build
make release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Using Make
make test
make test-verbose
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-features -- -D warnings

# Run all checks
make check
```

### Smoke Tests

```bash
# Run comprehensive smoke tests
make smoke
```

This will run:
- Build verification
- All unit and integration tests
- Clippy linting

## CI/CD

The project uses GitHub Actions for continuous integration:

- **CI Pipeline** (`.github/workflows/ci.yml`):
  - Code formatting checks
  - Clippy linting
  - Test execution
  - Build verification

- **Release Pipeline** (`.github/workflows/release.yml`):
  - Multi-platform builds (Linux, macOS)
  - Automated GitHub releases
  - Binary artifact publishing

## Configuration

Environment variables:

- `PORT`: Server port (default: 3000)
- `RUST_LOG`: Logging level (default: info)

Example:
```bash
PORT=8080 RUST_LOG=debug cargo run
```

## Agent Framework

The agent system is built on a trait-based architecture:

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    async fn handle(&self, input: Value) -> Result<Value>;
}
```

### Implementing a Custom Agent

```rust
use async_trait::async_trait;
use anyhow::Result;
use serde_json::{json, Value};
use crate::agent::Agent;

pub struct MyAgent;

#[async_trait]
impl Agent for MyAgent {
    async fn handle(&self, input: Value) -> Result<Value> {
        // Your agent logic here
        Ok(json!({
            "response": "Task completed",
            "status": "success"
        }))
    }
}
```

## Workflow Engine

The workflow engine provides FSM-based orchestration:

```rust
use crate::workflows::WorkflowEngine;
use crate::claude::ClaudeAgent;
use serde_json::json;

let engine = WorkflowEngine::new();
let agent = ClaudeAgent::new();
let input = json!({"task": "process"});

let next_state = engine.execute(&agent, "init", input).await?;
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting (`make check`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Enhanced agent implementations
- [ ] WebSocket support for real-time communication
- [ ] Metrics and observability integration
- [ ] Advanced workflow patterns
- [ ] Plugin system for extensibility
- [ ] Performance benchmarks and optimizations

## Support

For issues, questions, or contributions, please:
- Open an issue on GitHub
- Check existing documentation
- Review the examples in the codebase

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum)
- Async runtime by [Tokio](https://tokio.rs)
- Inspired by the MCP specification

---

**Maintained by:** eyshoit-commits