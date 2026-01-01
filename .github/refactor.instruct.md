# Refactor Instructions for Claude Code Router

## Objective
This document captures instructions to rebrand and refactor `Claude Code Router` into a Rust web framework.

## Deliverables
- A Rust web application replacing the existing Node.js architecture.
- Updated routes matching the previous functionality.

## Steps

1. **Preparation**:
    Ensure your working directory is clean before starting:
    ```bash
    git checkout main
    git pull origin main
    git checkout -b feature/rebrand-rust-router
    ```

2. **Implementation**:
    - Replace Node.js server:
        ```bash
        cargo new --bin rust-router
        cd rust-router
        ```

    - Define all routes and handlers using a Rust web framework, e.g., Actix Web.

3. **Verification**:
    - Verify server endpoints:
        ```curl
        curl http://localhost:8080/v1/messages/count_tokens
        ```
    - Test integration with the sandbox:
        ```bash
        make start-sandbox
        make smoke
        ```

4. **Documentation**:
    - Update `README.md` and detailed guides in `docs/` folder.

5. **Code Review & Merge**:
    - Ensure all changes are verified before submitting the pull request.
    - Assign the PR to relevant reviewers.
