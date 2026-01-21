#!/bin/bash

echo "🧪 ════════════════════════════════════════════════════════"
echo "🧪  Running Rust Backend Tests"
echo "🧪 ════════════════════════════════════════════════════════"

# Run cargo tests
echo ""
echo "🦀 Running unit tests..."
cargo test --lib -- --nocapture

echo ""
echo "🔗 Running integration tests (without AntTP)..."
cargo test --test '*' -- --skip ignored

echo ""
echo "📊 Test Summary:"
cargo test --lib --no-fail-fast 2>&1 | grep -E "test result:|running"

echo ""
echo "✅ To run tests that require AntTP (start services first):"
echo "   cargo test -- --ignored"
echo ""
echo "🦀 ════════════════════════════════════════════════════════"
