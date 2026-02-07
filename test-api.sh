#!/bin/bash

# AntTP Tutorial - API Test Script
# Tests all backend endpoints to verify functionality

set -e

BACKEND_URL="http://localhost:8080"
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🧪 AntTP Tutorial - API Test Suite"
echo "===================================="
echo ""

# Test counter
PASSED=0
FAILED=0

# Function to test an endpoint
test_endpoint() {
    local name=$1
    local method=$2
    local endpoint=$3
    local data=$4
    local expected_status=${5:-200}
    
    echo -n "Testing $name... "
    
    if [ -n "$data" ]; then
        response=$(curl -s -w "\n%{http_code}" -X $method \
            -H "Content-Type: application/json" \
            -d "$data" \
            "$BACKEND_URL$endpoint")
    else
        response=$(curl -s -w "\n%{http_code}" -X $method "$BACKEND_URL$endpoint")
    fi
    
    status=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [ "$status" -eq "$expected_status" ]; then
        echo -e "${GREEN}✓ PASSED${NC} (HTTP $status)"
        PASSED=$((PASSED + 1))
        if [ -n "$body" ]; then
            echo "   Response: $(echo $body | jq -c . 2>/dev/null || echo $body)"
        fi
    else
        echo -e "${RED}✗ FAILED${NC} (Expected HTTP $expected_status, got $status)"
        FAILED=$((FAILED + 1))
        if [ -n "$body" ]; then
            echo "   Response: $body"
        fi
    fi
    echo ""
}

# Check if backend is running
echo "🔍 Checking if backend is running..."
if ! curl -s "$BACKEND_URL/health" > /dev/null; then
    echo -e "${RED}❌ Backend is not running on $BACKEND_URL${NC}"
    echo "   Please start the backend first: docker-compose up -d"
    exit 1
fi
echo -e "${GREEN}✅ Backend is running${NC}"
echo ""

# Health Check
echo "1️⃣  Health Check"
test_endpoint "Health Check" "GET" "/health"

# Chunks Tests
echo "2️⃣  Chunks API"
test_endpoint "Store Chunk" "POST" "/api/chunks" \
    '{"content":"Hello, AntTP! This is my first chunk."}'

test_endpoint "List Chunks" "GET" "/api/chunks"

# Store another chunk and get its address
echo -n "Storing test chunk... "
chunk_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"content":"Test chunk for retrieval"}' \
    "$BACKEND_URL/api/chunks")
chunk_address=$(echo $chunk_response | jq -r '.address')
echo "Address: $chunk_address"
echo ""

if [ "$chunk_address" != "null" ] && [ -n "$chunk_address" ]; then
    test_endpoint "Get Chunk by Address" "GET" "/api/chunks/$chunk_address"
else
    echo -e "${YELLOW}⚠ Skipping chunk retrieval test (no address)${NC}"
    echo ""
fi

# Scratchpads Tests
echo "3️⃣  Scratchpads API"
test_endpoint "Create Public Scratchpad" "POST" "/api/scratchpads" \
    '{"name":"test-public","content":"Public scratchpad content","is_public":true}'

test_endpoint "Create Private Scratchpad" "POST" "/api/scratchpads" \
    '{"name":"test-private","content":"Private scratchpad content","is_public":false}'

test_endpoint "List Scratchpads" "GET" "/api/scratchpads"

test_endpoint "Get Scratchpad by Name" "GET" "/api/scratchpads/test-public"

test_endpoint "Update Scratchpad" "PUT" "/api/scratchpads/test-public" \
    '{"name":"test-public","content":"Updated public content","is_public":true}'

# Registers Tests
echo "4️⃣  Registers API"
test_endpoint "Set Register (New)" "POST" "/api/registers" \
    '{"key":"user:123:settings","value":"{\"theme\":\"dark\",\"lang\":\"en\"}"}'

test_endpoint "Set Register (Update)" "POST" "/api/registers" \
    '{"key":"user:123:settings","value":"{\"theme\":\"light\",\"lang\":\"fr\"}"}'

test_endpoint "List Registers" "GET" "/api/registers"

test_endpoint "Get Register by Key" "GET" "/api/registers/user:123:settings"

# PNR Tests
echo "5️⃣  PNR API"
test_endpoint "Create PNR Entry" "POST" "/api/pnr" \
    '{"name":"my-website","address":"chunk_abc123def456"}'

test_endpoint "Create Another PNR Entry" "POST" "/api/pnr" \
    '{"name":"my-profile","address":"scratchpad_xyz789"}'

test_endpoint "List PNR Entries" "GET" "/api/pnr"

test_endpoint "Resolve PNR Name" "GET" "/api/pnr/my-website"

test_endpoint "Resolve Non-existent Name" "GET" "/api/pnr/does-not-exist" "" 404

# Summary
echo "===================================="
echo "📊 Test Summary"
echo "===================================="
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo "Total:  $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi
