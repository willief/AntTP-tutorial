#!/bin/bash
echo "=== Autonomi Stack Diagnostics ==="
echo ""

echo "1. Docker Containers:"
docker ps -a | grep -E "(anttp|autonomi)" || echo "No containers found"

echo ""
echo "2. Port Scan:"
for port in 18888 8080 8000 5173; do
    echo -n "  Port $port: "
    timeout 1 bash -c "echo > /dev/tcp/localhost/$port" 2>/dev/null && echo "✓ OPEN" || echo "✗ CLOSED"
done

echo ""
echo "3. AntTP Logs (last 5 lines):"
docker logs --tail 5 anttp 2>/dev/null || echo "AntTP container not found"

echo ""
echo "4. Network Interfaces:"
ip -4 addr show | grep inet | awk '{print $2}' | grep -v "127.0.0.1"

echo ""
echo "5. Test Connections:"
echo -n "  AntTP health: "
curl --max-time 2 -s http://localhost:18888/health > /dev/null && echo "✓" || echo "✗"

echo -n "  Backend health: "
curl --max-time 2 -s http://localhost:8000/health > /dev/null && echo "✓" || echo "✗"

echo -n "  Frontend: "
curl --max-time 2 -s http://localhost:5173 > /dev/null && echo "✓" || echo "✗"

echo ""
echo "6. Container IPs:"
docker inspect anttp 2>/dev/null | grep IPAddress || echo "AntTP not running"
docker inspect autonomi-backend 2>/dev/null | grep IPAddress || echo "Backend not running"

echo ""
echo "=== Recommendations ==="
if ! curl --max-time 2 http://localhost:18888/health > /dev/null 2>&1; then
    echo "1. AntTP not responding. Check: docker logs anttp"
    echo "2. Try: docker-compose -f docker-compose.anttp.yml up -d"
elif ! curl --max-time 2 http://localhost:8000/health > /dev/null 2>&1; then
    echo "1. Backend not responding. Check: docker logs autonomi-backend"
    echo "2. Ensure ANTP_BASE_URL is correct in backend config"
else
    echo "All services appear to be running!"
    echo "Access:"
    echo "  - AntTP: http://localhost:18888/docs"
    echo "  - Backend: http://localhost:8000/docs"
    echo "  - Frontend: http://localhost:5173"
fi
