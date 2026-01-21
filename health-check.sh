#!/bin/bash
echo "=== Autonomi Stack Health Check ==="

# Check containers
echo "1. Containers:"
docker-compose ps

# Test endpoints
echo -e "\n2. Endpoint Health:"
echo -n "AntTP (18888): "
curl --max-time 2 -s http://localhost:18888/health > /dev/null && echo "✓" || echo "✗"

echo -n "Rust Backend (8000): "
curl --max-time 2 -s http://localhost:8000/health > /dev/null && echo "✓" || echo "✗"

echo -n "Frontend (5173): "
curl --max-time 2 -s http://localhost:5173 > /dev/null && echo "✓" || echo "✗"

# Check connectivity between services
echo -e "\n3. Inter-service connectivity:"
echo -n "Rust Backend → AntTP: "
docker-compose exec rust-backend sh -c "nc -z anttp 18888 2>/dev/null && echo '✓' || echo '✗'" 2>/dev/null || echo "Test failed"

# Display URLs
echo -e "\n4. Access URLs:"
echo "   AntTP API:      http://localhost:18888/docs"
echo "   Rust Backend:   http://localhost:8000/"
echo "   Frontend:       http://localhost:5173"
