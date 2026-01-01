.PHONY: help build test clean run dev fmt lint check docker-build docker-run smoke install start-rust-router

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install dependencies
	cargo fetch

build: ## Build the project
	cargo build

release: ## Build release version
	cargo build --release

test: ## Run tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture --test-threads=1

clean: ## Clean build artifacts
	cargo clean

run: ## Run the server
	cargo run

start-rust-router: build ## Start Rust Web Router
	cargo run

dev: ## Run the server with auto-reload (requires cargo-watch)
	cargo watch -x run

fmt: ## Format code
	cargo fmt --all

lint: ## Run clippy
	cargo clippy --all-features -- -D warnings

check: fmt lint test ## Run all checks (format, lint, test)

docker-build: ## Build Docker image
	docker build -t rust-web-router:latest .

docker-run: ## Run Docker container
	docker run -p 8080:8080 --rm rust-web-router:latest

docker-compose-up: ## Start services with docker-compose
	docker-compose up -d

docker-compose-down: ## Stop services with docker-compose
	docker-compose down

start-ccc: build ## Start CCC server (alias for start-rust-router)
	cargo run

smoke: ## Run smoke tests
	@echo "Running smoke tests..."
	@cargo build
	@echo "✓ Build successful"
	@cargo test
	@echo "✓ Tests passed"
	@cargo clippy -- -D warnings
	@echo "✓ Clippy checks passed"
	@echo "All smoke tests passed!"

watch: ## Watch for changes and run tests
	cargo watch -x test
