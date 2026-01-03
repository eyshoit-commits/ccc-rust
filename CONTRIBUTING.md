# Contributing to CCC Rust MCP

Thank you for your interest in contributing to the CCC Rust MCP project! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/ccc-rust.git
   cd ccc-rust
   ```
3. **Set up the development environment**:
   ```bash
   # Install Rust if you haven't already
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install dependencies
   cargo fetch
   
   # Run tests to ensure everything works
   make test
   ```

## Development Workflow

### 1. Create a Branch

Create a feature branch from `main`:
```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write clean, idiomatic Rust code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Run Quality Checks

Before committing, ensure your code passes all checks:

```bash
# Format code
make fmt

# Run linter
make lint

# Run tests
make test

# Or run all checks at once
make check
```

### 4. Commit Your Changes

Write clear, descriptive commit messages:
```bash
git add .
git commit -m "Add feature: brief description of changes"
```

Follow these commit message guidelines:
- Use the imperative mood ("Add feature" not "Added feature")
- Keep the first line under 72 characters
- Add detailed explanation in the body if needed

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear title describing the change
- Description of what changed and why
- Reference to any related issues

## Coding Standards

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Ensure `cargo clippy` passes with no warnings
- Write idiomatic Rust code

### Code Organization

- Keep modules focused and single-purpose
- Use meaningful names for functions and variables
- Add doc comments for public APIs
- Keep functions small and focused

### Testing

- Write unit tests for new functionality
- Add integration tests for API endpoints
- Ensure all tests pass before submitting PR
- Aim for high test coverage

Example test:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_feature() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = my_function(input).await;
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }
}
```

### Documentation

- Add doc comments (`///`) for public items
- Include examples in documentation when helpful
- Update README.md for significant changes
- Keep CHANGELOG.md updated

Example documentation:
```rust
/// Processes the given task and returns a result.
///
/// # Arguments
///
/// * `task` - The task to process
///
/// # Returns
///
/// A `Result` containing the processed output or an error
///
/// # Examples
///
/// ```
/// let result = process_task(task).await?;
/// ```
pub async fn process_task(task: Task) -> Result<Output> {
    // Implementation
}
```

## Pull Request Process

1. **Update Documentation**: Ensure README and code comments are current
2. **Add Tests**: All new features must include tests
3. **Run Checks**: All CI checks must pass
4. **Request Review**: Tag maintainers for review
5. **Address Feedback**: Respond to review comments promptly
6. **Squash Commits**: Consider squashing commits before merge

### PR Checklist

Before submitting a PR, verify:

- [ ] Code follows Rust style guidelines
- [ ] `cargo fmt` has been run
- [ ] `cargo clippy` passes with no warnings
- [ ] All tests pass (`cargo test`)
- [ ] New tests added for new functionality
- [ ] Documentation updated as needed
- [ ] Commit messages are clear and descriptive
- [ ] No unnecessary files committed

## Reporting Bugs

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Detailed steps to reproduce the bug
3. **Expected Behavior**: What you expected to happen
4. **Actual Behavior**: What actually happened
5. **Environment**: 
   - OS and version
   - Rust version (`rustc --version`)
   - Relevant dependencies
6. **Logs**: Error messages or logs if available

## Feature Requests

Feature requests are welcome! Please:

1. Check if the feature already exists or is planned
2. Describe the use case and benefits
3. Provide examples of how it would be used
4. Consider implementation approach (if applicable)

## Development Tips

### Using devcontainer

For a consistent development environment:

1. Open project in VSCode
2. Install "Remote - Containers" extension
3. Click "Reopen in Container" when prompted
4. All tools will be pre-configured

### Useful Commands

```bash
# Build
make build

# Run with auto-reload
make dev

# Run specific test
cargo test test_name

# Check without building
cargo check

# Build documentation
cargo doc --open

# Run in release mode
cargo run --release
```

### Debugging

- Use `RUST_LOG=debug` for verbose logging
- Add `#[tokio::test]` for async tests
- Use VSCode debugger with included launch configuration

## Questions?

If you have questions:

1. Check existing documentation
2. Search through issues
3. Open a new issue with the "question" label
4. Join discussions in pull requests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to CCC Rust MCP! 🦀
