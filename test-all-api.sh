#!/bin/bash

# AntTP Tutorial - Complete API Test Suite
# Tests all storage primitives: Chunks, Files, Registers, Pointers, Archives, PNR

set -e

BACKEND_URL="http://localhost:8080"
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🧪 AntTP Tutorial - Complete API Test Suite"
echo "=============================================="
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
    echo "   Please start the backend first: docker compose up -d"
    exit 1
fi
echo -e "${GREEN}✅ Backend is running${NC}"
echo ""

# Health Check
echo -e "${BLUE}1️⃣  Health Check${NC}"
test_endpoint "Health Check" "GET" "/health"

# Chunks Tests
echo -e "${BLUE}2️⃣  Chunks API${NC}"
test_endpoint "Store Chunk #1" "POST" "/api/chunks" \
    '{"content":"Hello, AntTP! This is immutable data."}'

test_endpoint "Store Chunk #2" "POST" "/api/chunks" \
    '{"content":"Another chunk with different content."}'

test_endpoint "List Chunks" "GET" "/api/chunks"

# Store a chunk and get its address for retrieval
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
fi

# Files Tests
echo -e "${BLUE}3️⃣  Files API${NC}"
test_endpoint "Upload File (text)" "POST" "/api/files" \
    '{"name":"document.txt","content":"This is a text file content","content_type":"text/plain"}'

test_endpoint "Upload File (json)" "POST" "/api/files" \
    '{"name":"data.json","content":"{\"key\":\"value\",\"number\":42}","content_type":"application/json"}'

test_endpoint "List Files" "GET" "/api/files"

# Upload a file and get its datamap for retrieval
echo -n "Uploading test file... "
file_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"test.dat","content":"Test file data for retrieval"}' \
    "$BACKEND_URL/api/files")
file_datamap=$(echo $file_response | jq -r '.data_map')
echo "DataMap: $file_datamap"
echo ""

if [ "$file_datamap" != "null" ] && [ -n "$file_datamap" ]; then
    test_endpoint "Get File by DataMap" "GET" "/api/files/$file_datamap"
fi

# Registers Tests
echo -e "${BLUE}4️⃣  Registers API${NC}"
test_endpoint "Set Register (Create)" "POST" "/api/registers" \
    '{"key":"app:config","value":"{\"theme\":\"dark\",\"notifications\":true}"}'

test_endpoint "Set Register (Update)" "POST" "/api/registers" \
    '{"key":"app:config","value":"{\"theme\":\"light\",\"notifications\":false}"}'

test_endpoint "Set Register (User Profile)" "POST" "/api/registers" \
    '{"key":"user:alice:profile","value":"{\"name\":\"Alice\",\"level\":42}"}'

test_endpoint "List Registers" "GET" "/api/registers"

test_endpoint "Get Register by Key" "GET" "/api/registers/app:config"

# Pointers Tests
echo -e "${BLUE}5️⃣  Pointers API${NC}"
test_endpoint "Create Pointer" "POST" "/api/pointers" \
    '{"name":"latest-version","target":"chunk_abc123"}'

test_endpoint "Create Another Pointer" "POST" "/api/pointers" \
    '{"name":"current-config","target":"register_def456"}'

test_endpoint "Update Pointer" "PUT" "/api/pointers/latest-version" \
    '{"name":"latest-version","target":"chunk_xyz789"}'

test_endpoint "List Pointers" "GET" "/api/pointers"

test_endpoint "Get Pointer by Name" "GET" "/api/pointers/latest-version"

# Archives Tests
echo -e "${BLUE}6️⃣  Archives API${NC}"
test_endpoint "Create Archive (Website)" "POST" "/api/archives" \
    '{"name":"my-website","files":[{"path":"index.html","content":"<html><body>Hello</body></html>"},{"path":"style.css","content":"body { color: blue; }"}],"metadata":{"author":"Alice","version":"1.0"}}'

test_endpoint "Create Archive (Photos)" "POST" "/api/archives" \
    '{"name":"vacation-2024","files":[{"path":"beach.jpg","content":"[binary data]"},{"path":"sunset.jpg","content":"[binary data]"}]}'

test_endpoint "List Archives" "GET" "/api/archives"

# Create an archive and get its address for retrieval
echo -n "Creating test archive... "
archive_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"test-archive","files":[{"path":"file1.txt","content":"content1"}]}' \
    "$BACKEND_URL/api/archives")
archive_address=$(echo $archive_response | jq -r '.archive.address')
echo "Address: $archive_address"
echo ""

if [ "$archive_address" != "null" ] && [ -n "$archive_address" ]; then
    test_endpoint "Get Archive by Address" "GET" "/api/archives/$archive_address"
fi

# PNR Tests
echo -e "${BLUE}7️⃣  PNR (Personal Name Resolution) API${NC}"
test_endpoint "Create PNR (Chunk)" "POST" "/api/pnr" \
    '{"name":"my-homepage","target":"chunk_abc123","record_type":"chunk"}'

test_endpoint "Create PNR (File)" "POST" "/api/pnr" \
    '{"name":"profile-photo","target":"datamap_xyz789","record_type":"file"}'

test_endpoint "Create PNR (Archive)" "POST" "/api/pnr" \
    '{"name":"my-website","target":"archive_def456","record_type":"archive"}'

test_endpoint "List PNR Entries" "GET" "/api/pnr"

test_endpoint "Resolve PNR Name" "GET" "/api/pnr/my-homepage"

test_endpoint "Resolve Non-existent Name" "GET" "/api/pnr/does-not-exist" "" 404

# Summary
echo "=============================================="
echo -e "${YELLOW}📊 Test Summary${NC}"
echo "=============================================="
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo "Total:  $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    echo ""
    echo "✅ All Autonomi primitives working:"
    echo "   📦 Chunks - Immutable content storage"
    echo "   📁 Files - Large data with chunking"
    echo "   📊 Registers - Versioned key-value storage"
    echo "   👉 Pointers - Mutable references"
    echo "   📚 Archives - File collections"
    echo "   🏷️  PNR - Human-readable names"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi
