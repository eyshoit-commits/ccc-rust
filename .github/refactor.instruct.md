# Refactor Instructions

## Overview
This document outlines the refactoring from "Claude Code Router" to "Rust Web Framework" with migration from Axum to Actix-web.

## Changes Made

### 1. Framework Migration
- **From**: Axum web framework
- **To**: Actix-web 4.x
- **Reason**: Better ecosystem integration and more mature middleware support

### 2. Rebranding
- **Old Name**: CCC Rust MCP (Claude Code Router)
- **New Name**: Rust Web Router (Rust Web Framework)
- **Package Name**: `rust-web-router`
- **Binary Name**: `rust-web-router`

### 3. Dependency Changes

#### Removed
- `axum` - Web framework
- `tower` - Middleware layer
- `tower-http` - HTTP utilities
- `tokio` (as explicit dependency, now via actix-rt)

#### Added
- `actix-web` - Web framework
- `actix-rt` - Runtime
- `actix-cors` - CORS middleware
- `env_logger` - Logging

### 4. API Changes

#### Endpoints (Unchanged)
- `GET /` - Welcome message
- `GET /health` - Health check
- `POST /v1/messages/count_tokens` - Token counting
- `POST /v1/mcp/route` - MCP task routing

#### Port Changes
- **Old**: Default port 3000
- **New**: Default port 8080
- **Binding**: Changed from `0.0.0.0` to `127.0.0.1` for security

### 5. Code Structure Changes

#### main.rs
- Changed from `#[tokio::main]` to `#[actix_web::main]`
- Updated to use Actix-web's `HttpServer` and `App`
- State management now uses `web::Data<AppState>`

#### mcp_server.rs
- Replaced Axum handlers with Actix-web handlers
- Changed from `Json<T>` to `web::Json<T>`
- Updated response types to use `HttpResponse`
- Replaced `Router` with route configuration function

#### Tests
- Updated to use Actix-web test utilities
- Changed from `tower::ServiceExt` to `actix_web::test`

### 6. Makefile Updates

#### New Targets
- `start-rust-router` - Start the Rust Web Router server

#### Updated Targets
- `docker-build` - Now builds `rust-web-router:latest`
- `docker-run` - Updated to use port 8080

### 7. Configuration Changes

#### Environment Variables
- `PORT` - Server port (default: 8080, was 3000)
- `RUST_LOG` - Logging level (updated prefix to `rust_web_router`)

### 8. Documentation Updates

Files to update:
- `README.md` - Update all references to new framework name
- `AGENTS.md` - Update implementation details
- `CHANGELOG.md` - Add migration notes
- `CONTRIBUTING.md` - Update build/run instructions
- `docker-compose.yml` - Update service name and ports
- `Dockerfile` - Update binary name
- `.env.example` - Update environment variables

### 9. GitHub Workflows

Files to update:
- `.github/workflows/ci.yml` - Update binary artifact name
- `.github/workflows/release.yml` - Update release asset names

## Migration Guide

### For Developers

1. **Clean old build**:
   ```bash
   cargo clean
   ```

2. **Update dependencies**:
   ```bash
   cargo fetch
   ```

3. **Build new version**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Start server**:
   ```bash
   make start-rust-router
   # or
   cargo run
   ```

### For Users

The API endpoints remain the same, but:
- Default port changed from 3000 to 8080
- Update your client configurations accordingly
- Docker image name changed to `rust-web-router`

## Compatibility Notes

- All API endpoints remain backward compatible
- Response formats unchanged
- Only infrastructure changes (framework, ports, names)

## Testing Checklist

- [x] All unit tests pass
- [x] All integration tests pass
- [x] Clippy checks pass
- [x] Format checks pass
- [x] Docker build succeeds
- [ ] CI/CD pipeline updated
- [ ] Documentation updated

## Future Work

- Consider workspace structure for multi-crate projects
- Add more middleware (rate limiting, authentication)
- Enhance logging with structured logging
- Add metrics and observability

---

**Migration Date**: 2026-01-01
**Migrated By**: @copilot
**Reviewed By**: TBD
