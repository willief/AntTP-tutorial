#!/bin/bash

echo "🚀 Starting Autonomi AntTP Explorer..."
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Error: Docker is not running"
    echo "Please start Docker and try again"
    exit 1
fi

echo "✅ Docker is running"
echo ""

# Create .env from example if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from .env.example..."
    cp .env.example .env
fi

echo "🐳 Starting Docker containers..."
docker compose up -d 2>/dev/null || docker-compose up -d

echo ""
echo "⏱️  IMPORTANT: Waiting for AntTP to initialize..."
echo ""
echo "AntTP needs 30-60 seconds to start. This is normal!"
echo "Services are running, but won't respond immediately."
echo ""

# Simple countdown
echo "Waiting 60 seconds for AntTP to initialize..."
for i in {60..1}; do
    printf "\r   %2d seconds remaining... " $i
    sleep 1
done
echo ""
echo ""

# Quick check if services are responding
echo "🔍 Checking service status..."
echo ""

if curl -s http://localhost:18888 >/dev/null 2>&1; then
    echo "✅ AntTP is responding on port 18888"
else
    echo "⚠️  AntTP not responding yet (may need more time)"
fi

if curl -s http://localhost:8000/health >/dev/null 2>&1; then
    echo "✅ Backend is responding on port 8000"
else
    echo "⚠️  Backend not responding yet (may need more time)"
fi

echo "📝 Frontend is starting on port 5173"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Services Started!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📍 Access URLs:"
echo "   Frontend:  http://localhost:5173"
echo "   Backend:   http://localhost:8000/docs"
echo "   AntTP:     http://localhost:18888"
echo ""
echo "⚠️  If services don't respond:"
echo "   1. Wait another 30-60 seconds"
echo "   2. Check logs: docker compose logs -f anttp"
echo "   3. Look for: 'Listening on http://0.0.0.0:18888'"
echo ""
echo "📚 Documentation:"
echo "   - QUICKSTART.md     - Getting started"
echo "   - TUTORIAL.md       - Feature guides"
echo "   - TROUBLESHOOTING.md - Common issues"
echo ""
echo "🛑 To stop: docker compose down"
echo ""
