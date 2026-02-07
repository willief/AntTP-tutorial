#!/bin/bash

# Backend diagnostic script

echo "🔍 Backend Container Diagnostics"
echo "================================="
echo ""

echo "📊 Container Status:"
docker compose ps backend
echo ""

echo "📝 Recent Logs:"
docker compose logs backend --tail=50
echo ""

echo "🐚 Checking if binary exists in container:"
docker compose exec -T backend ls -lh /app/anttp-tutorial-backend 2>&1 || echo "Cannot exec - container not running"
echo ""

echo "🔍 Checking binary architecture:"
docker compose exec -T backend file /app/anttp-tutorial-backend 2>&1 || echo "Cannot check - container not running"
echo ""

echo "🧪 Trying to run binary manually:"
docker compose exec -T backend /app/anttp-tutorial-backend 2>&1 || echo "Cannot run - container not running or binary failed"
echo ""

echo "📋 Container events:"
docker compose events --tail=20 backend &
sleep 2
kill %1 2>/dev/null
echo ""

echo "💡 Quick fix: Run './rebuild.sh' to rebuild everything fresh"
