#!/bin/bash

echo "🦀 ════════════════════════════════════════════════════════"
echo "🦀  AntTP-Compatible Rust Backend - Quick Start"
echo "🦀 ════════════════════════════════════════════════════════"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust/Cargo found: $(cargo --version)"
echo ""

# Create .env if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env from .env.example..."
    cp .env.example .env
    echo "✅ .env created"
    echo "⚠️  Using default testnet configuration"
    echo ""
fi

# Build project
echo "🔨 Building project (this may take a few minutes first time)..."
cargo build --release

if [ $? -ne 0 ]; then
    echo ""
    echo "❌ Build failed. Check error messages above."
    exit 1
fi

echo ""
echo "✅ Build successful!"
echo ""
echo "🚀 Starting server..."
echo ""

# Run the server
cargo run --release
