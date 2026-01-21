#!/bin/bash
echo "=== COMPLETE STACK VERIFICATION ==="

ALL_OK=true

# Check 1: All containers running
echo ""
echo "1. Container Status:"
CONTAINERS=$(docker-compose ps -q | wc -l)
if [ "$CONTAINERS" -eq 3 ]; then
  echo "   ✅ All 3 containers are running"
  docker-compose ps --format "table {{.Name}}\t{{.Status}}"
else
  echo "   ❌ Only $CONTAINERS/3 containers running"
  ALL_OK=false
fi

# Check 2: Port responses
echo ""
echo "2. Port Responses:"
PORTS="18888 8000 5173"
for port in $PORTS; do
  if timeout 2 bash -c "echo > /dev/tcp/localhost/$port" 2>/dev/null; then
    echo "   ✅ Port $port is responding"
  else
    echo "   ❌ Port $port is not responding"
    ALL_OK=false
  fi
done

# Check 3: HTTP endpoints
echo ""
echo "3. HTTP Health Checks:"
echo -n "   AntTP (/health): "
if curl -s --max-time 3 http://localhost:18888/health > /dev/null; then
  echo "✅ Healthy"
else
  echo "❌ Not responding"
  ALL_OK=false
fi

echo -n "   Rust Backend (/health): "
if curl -s --max-time 3 http://localhost:8000/health > /dev/null; then
  echo "✅ Healthy"
else
  echo "❌ Not responding"
  ALL_OK=false
fi

echo -n "   Frontend (/): "
if curl -s --max-time 3 http://localhost:5173 > /dev/null; then
  echo "✅ Responding"
else
  echo "❌ Not responding"
  ALL_OK=false
fi

# Final result
echo ""
echo "====================================="
if [ "$ALL_OK" = true ]; then
  echo "🎉 SUCCESS: Autonomi Stack is fully operational!"
  echo ""
  echo "Access URLs:"
  echo "  • Frontend: http://localhost:5173"
  echo "  • Rust API: http://localhost:8000"
  echo "  • AntTP API: http://localhost:18888"
  echo "  • AntTP Health: http://localhost:18888/health"
  echo ""
  echo "Development stack is ready! 🚀"
else
  echo "⚠️  Some issues detected. Check logs with: docker-compose logs"
fi
