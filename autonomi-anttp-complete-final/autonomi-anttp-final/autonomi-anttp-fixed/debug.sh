#!/bin/bash

# Troubleshooting and Debug Helper Script

echo "🔍 Autonomi AntTP Explorer - Troubleshooting Helper"
echo ""

# Function to print section headers
section() {
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "$1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

# Check Docker
section "🐳 Docker Status"
if docker info >/dev/null 2>&1; then
    echo "✅ Docker is running"
    docker --version
else
    echo "❌ Docker is not running"
    echo "Please start Docker Desktop"
    exit 1
fi

# Check containers
section "📦 Container Status"
docker compose ps

# Check if AntTP is responding
section "🌐 Service Health Checks"

echo -n "AntTP (port 18888): "
if curl -s http://localhost:18888 >/dev/null 2>&1; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
    echo "   Try: docker compose logs anttp | tail -20"
fi

echo -n "Backend (port 8000): "
if curl -s http://localhost:8000/health >/dev/null 2>&1; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
    echo "   Try: docker compose logs backend | tail -20"
fi

echo -n "Frontend (port 5173): "
if curl -s http://localhost:5173 >/dev/null 2>&1; then
    echo "✅ Responding"
else
    echo "❌ Not responding"
    echo "   Try: docker compose logs frontend | tail -20"
fi

# Check ports
section "🔌 Port Usage"
echo "Checking if required ports are available..."
echo ""

for port in 18888 8000 5173; do
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo "Port $port: ✅ In use (by Docker or another process)"
        lsof -Pi :$port -sTCP:LISTEN 2>/dev/null || echo "   (Process details unavailable)"
    else
        echo "Port $port: ⚠️  Not in use (container may not be running)"
    fi
done

# Recent logs
section "📝 Recent Logs (Last 20 Lines)"

echo "=== AntTP Logs ==="
docker compose logs anttp | tail -20

echo ""
echo "=== Backend Logs ==="
docker compose logs backend | tail -20

echo ""
echo "=== Frontend Logs ==="
docker compose logs frontend | tail -20

# Environment check
section "🔧 Environment Configuration"

echo "Checking .env file..."
if [ -f .env ]; then
    echo "✅ .env exists"
    echo ""
    echo "Contents:"
    cat .env
else
    echo "❌ .env missing"
    echo "Run: cp .env.example .env"
fi

# Python venv check
section "🐍 Python Virtual Environment"

if [ -d backend/venv ]; then
    echo "✅ Virtual environment exists at backend/venv"
    echo ""
    echo "To use for local testing:"
    echo "  cd backend"
    echo "  source venv/bin/activate"
    echo "  pytest tests/ -v"
    echo "  deactivate"
else
    echo "⚠️  Virtual environment not created"
    echo ""
    echo "To create:"
    echo "  cd backend"
    echo "  python3 -m venv venv"
    echo "  source venv/bin/activate"
    echo "  pip install -r requirements.txt"
fi

# Useful commands
section "🛠️ Useful Troubleshooting Commands"

cat << 'EOF'
# View all logs in real-time:
docker compose logs -f

# View specific service logs:
docker compose logs -f anttp
docker compose logs -f backend
docker compose logs -f frontend

# Restart a specific service:
docker compose restart anttp
docker compose restart backend
docker compose restart frontend

# Stop everything:
docker compose down

# Stop and remove volumes (clean slate):
docker compose down -v

# Rebuild containers:
docker compose build --no-cache
docker compose up -d

# Access container shell:
docker compose exec backend bash
docker compose exec frontend sh

# Check backend directly inside container:
docker compose exec backend curl http://anttp:18888

# Run backend tests locally:
cd backend && source venv/bin/activate && pytest tests/ -v

# Run frontend tests locally:
cd frontend && npm test
EOF

echo ""
section "📚 Additional Resources"

cat << 'EOF'
- QUICKSTART.md         - Quick start guide
- TROUBLESHOOTING.md    - Detailed troubleshooting
- DEVELOPMENT.md        - Development workflows
- TUTORIAL.md           - AntTP features guide

For more help:
- Check logs: docker compose logs -f
- Restart services: docker compose restart
- Clean restart: docker compose down -v && docker compose up -d
EOF

echo ""
