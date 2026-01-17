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

# Setup Python virtual environment for local testing/troubleshooting
if [ ! -d "backend/venv" ]; then
    echo "🐍 Setting up Python virtual environment for local testing..."
    echo "   (This is for running tests locally, not required for Docker)"
    
    if command -v python3 &> /dev/null; then
        cd backend
        python3 -m venv venv
        
        # Activate and install dependencies
        source venv/bin/activate
        pip install --quiet --upgrade pip
        pip install --quiet -r requirements.txt
        deactivate
        cd ..
        echo "   ✅ Virtual environment created at backend/venv"
        echo "   To use: cd backend && source venv/bin/activate"
    else
        echo "   ⚠️  Python3 not found - skipping venv creation"
        echo "   Install Python3 to enable local testing"
    fi
    echo ""
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
echo "🧪 Local Testing (without Docker):"
echo "   Backend tests:"
echo "     cd backend && source venv/bin/activate"
echo "     pytest tests/ -v"
echo "     deactivate"
echo ""
echo "   Frontend tests:"
echo "     cd frontend"
echo "     npm install  # (first time only)"
echo "     npm test"
echo ""
echo "🛑 To stop: docker compose down"
echo ""
