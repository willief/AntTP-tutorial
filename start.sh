#!/bin/bash

echo "🦀 ════════════════════════════════════════════════════════"
echo "🦀  Autonomi Rust Backend - Quick Start"
echo "🦀 ════════════════════════════════════════════════════════"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Error: Docker is not running"
    echo "Please start Docker Desktop and try again"
    exit 1
fi

# Create .env if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env from .env.example..."
    cp .env.example .env
fi

# Start Docker Compose
echo "🚀 Starting services..."
docker compose up -d

# Wait for AntTP to be ready
echo "⏳ Waiting for AntTP to initialize (60 seconds)..."
for i in {60..1}; do
    echo -ne "   $i seconds remaining...\r"
    sleep 1
done
echo ""

echo ""
echo "✅ Services are ready!"
echo ""
echo "🌐 Access your application:"
echo "   Frontend:  http://localhost:5173"
echo "   Backend:   http://localhost:8000"
echo "   AntTP:     http://localhost:18888"
echo "   API Docs:  http://localhost:8000/health"
echo ""
echo "📊 View logs:"
echo "   docker compose logs -f"
echo ""
echo "🧪 Run tests:"
echo "   ./test.sh"
echo ""
echo "🦀 ════════════════════════════════════════════════════════"
