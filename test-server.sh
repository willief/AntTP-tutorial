#!/bin/bash
# test-server.sh - Quick test of running server

echo "🧪 Testing AntTP Backend Server"
echo "================================"
echo ""

BASE_URL="http://localhost:18888"

# Test 1: Health check
echo "1️⃣ Health Check"
curl -s $BASE_URL/health | jq
echo ""

# Test 2: Create chunk
echo "2️⃣ Create Chunk"
CHUNK_ADDR=$(curl -s -X POST $BASE_URL/anttp-0/chunk \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"content":"SGVsbG8gV29ybGQh"}' | jq -r '.address')
echo "   Created: $CHUNK_ADDR"
echo ""

# Test 3: Get chunk back
echo "3️⃣ Get Chunk"
curl -s "$BASE_URL/anttp-0/chunk/$CHUNK_ADDR" \
  -H 'x-store-type: memory' | jq
echo ""

# Test 4: Create register
echo "4️⃣ Create Register"
REG_ADDR=$(curl -s -X POST $BASE_URL/anttp-0/register \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"test","content":"68656c6c6f"}' | jq -r '.address')
echo "   Created: $REG_ADDR"
echo ""

# Test 5: Update register
echo "5️⃣ Update Register"
curl -s -X PUT "$BASE_URL/anttp-0/register/$REG_ADDR" \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"test","content":"776f726c64"}' | jq
echo ""

# Test 6: Get register history
echo "6️⃣ Register History"
curl -s "$BASE_URL/anttp-0/register_history/$REG_ADDR" \
  -H 'x-store-type: memory' | jq
echo ""

# Test 7: Commands list
echo "7️⃣ Available Commands"
curl -s $BASE_URL/anttp-0/command | jq '.total_endpoints'
echo ""

echo "================================"
echo "✅ All tests completed!"
echo ""
echo "💡 Tip: Base64 'SGVsbG8gV29ybGQh' = 'Hello World!'"
echo "💡 Tip: Hex '68656c6c6f' = 'hello', '776f726c64' = 'world'"
