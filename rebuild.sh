#!/bin/bash

# Clean rebuild script for AntTP Tutorial

echo "🧹 Stopping and removing containers..."
docker compose down -v

echo "🗑️  Removing old images..."
docker rmi anttp-tutorial-backend anttp-tutorial-frontend 2>/dev/null || echo "   (No old images to remove)"

echo ""
echo "🏗️  Building fresh images (this may take a few minutes)..."
docker compose build --no-cache

echo ""
echo "🚀 Starting services..."
docker compose up -d

echo ""
echo "⏳ Waiting for services to initialize..."
sleep 8

echo ""
echo "📊 Container Status:"
docker compose ps

echo ""
echo "📝 Backend Logs (last 15 lines):"
docker compose logs backend | tail -15

echo ""
echo "📝 Frontend Logs (last 10 lines):"
docker compose logs frontend | tail -10

echo ""
echo "✅ Done! Check the services:"
echo "   Frontend: http://localhost:3000"
echo "   Backend:  http://localhost:8080/health"
echo ""
echo "💡 To follow logs: docker compose logs -f"

