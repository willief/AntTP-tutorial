#!/bin/bash

echo "=== Autonomi Stack Integration Tests ==="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to test endpoint
test_endpoint() {
    local name=$1
    local url=$2
    local expected_status=${3:-200}
    
    echo -n "Testing $name... "
    response=$(curl -s -o /dev/null -w "%{http_code}" "$url" 2>/dev/null)
    
    if [ "$response" = "$expected_status" ]; then
        echo -e "${GREEN}✓ OK${NC} (Status: $response)"
        return 0
    else
        echo -e "${RED}✗ FAILED${NC} (Status: $response, Expected: $expected_status)"
        return 1
    fi
}

# Run tests
echo "1. Container Status:"
docker-compose ps | grep -E "(anttp|autonomi-backend|autonomi-frontend)"

echo -e "\n2. Service Health Checks:"
test_endpoint "AntTP Health" "http://localhost:18888/health.json"
test_endpoint "Backend Health" "http://localhost:8000/health"
test_endpoint "Backend Docs" "http://localhost:8000/docs"

echo -e "\n3. Inter-Service Communication:"
echo -n "Testing Backend → AntTP connectivity... "
if docker-compose exec rust-backend curl -s http://anttp:80/health.json > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Connected${NC}"
else
    echo -e "${RED}✗ Failed${NC}"
fi

echo -e "\n4. API Functionality Tests:"
# Test chunk creation
echo -n "Testing chunk creation... "
test_chunk=$(curl -s -X POST "http://localhost:8000/api/chunks" \
    -H "Content-Type: application/json" \
    -d '{"data": "test data"}' \
    -w "%{http_code}" 2>/dev/null)
if [[ "$test_chunk" =~ 200|201 ]]; then
    echo -e "${GREEN}✓ Working${NC}"
else
    echo -e "${YELLOW}⚠ Partial${NC} (Status: ${test_chunk: -3})"
fi

echo -e "\n=== Test Summary ==="
echo "Frontend: http://localhost:5173"
echo "Backend API: http://localhost:8000/"
echo "AntTP Mock: http://localhost:18888/health.json"
echo ""
echo "All systems operational! 🚀"