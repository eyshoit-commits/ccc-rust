# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-01

### Added
- Initial release of CCC Rust MCP server
- Agent trait system for modular agent implementations
- ClaudeAgent reference implementation
- Workflow engine with FSM-based orchestration
- MCP server with Axum framework
- HTTP endpoints:
  - `GET /health` - Health check
  - `POST /v1/messages/count_tokens` - Token counting
  - `POST /v1/mcp/route` - MCP task routing
- MCP adapter for sandbox-mcp integration
- Comprehensive test suite with 5 passing tests
- GitHub Actions CI/CD workflows:
  - Continuous integration (build, test, lint, format)
  - Multi-platform release automation
- Docker support:
  - Multi-stage Dockerfile
  - Docker Compose configuration
  - Health checks
- Development environment:
  - VSCode devcontainer with Rust toolchain
  - Pre-configured extensions and settings
  - Docker-in-Docker support
- Documentation:
  - Comprehensive README with examples
  - API documentation
  - Contributing guidelines
  - License (MIT)
- Build automation:
  - Makefile with common commands
  - Smoke test suite
  - Format and lint scripts

### Changed
- N/A (initial release)

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- Non-root user in Docker container
- Minimal base image (Debian Bookworm Slim)

## [Unreleased]

### Planned
- Enhanced agent implementations
- WebSocket support for real-time communication
- Metrics and observability integration
- Advanced workflow patterns
- Plugin system for extensibility
- Performance benchmarks and optimizations

---

[0.1.0]: https://github.com/eyshoit-commits/ccc-rust/releases/tag/v0.1.0
