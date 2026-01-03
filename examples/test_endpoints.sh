#!/usr/bin/env bash
# Example script to test the MCP server endpoints

set -e

BASE_URL="${BASE_URL:-http://localhost:3000}"

echo "Testing CCC Rust MCP Server at $BASE_URL"
echo "=========================================="

# Test health endpoint
echo ""
echo "1. Testing health endpoint..."
curl -s "$BASE_URL/health" | jq '.'

# Test token counting
echo ""
echo "2. Testing token counting..."
curl -s -X POST "$BASE_URL/v1/messages/count_tokens" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "The quick brown fox jumps over the lazy dog"
  }' | jq '.'

# Test MCP routing with simple task
echo ""
echo "3. Testing MCP routing (simple task)..."
curl -s -X POST "$BASE_URL/v1/mcp/route" \
  -H "Content-Type: application/json" \
  -d '{
    "task": "translate",
    "context": {
      "source_lang": "en",
      "target_lang": "es",
      "text": "Hello, world!"
    }
  }' | jq '.'

# Test MCP routing with complex task
echo ""
echo "4. Testing MCP routing (complex task)..."
curl -s -X POST "$BASE_URL/v1/mcp/route" \
  -H "Content-Type: application/json" \
  -d '{
    "task": "analyze_code",
    "context": {
      "language": "rust",
      "code": "fn main() { println!(\"Hello\"); }",
      "analysis_type": "security"
    }
  }' | jq '.'

echo ""
echo "=========================================="
echo "All tests completed successfully!"
