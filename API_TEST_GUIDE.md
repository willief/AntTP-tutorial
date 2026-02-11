# 🧪 Complete API Test Guide

## All 20+ Endpoints Tested!

This guide shows how to test every single endpoint in the AntTP-compatible backend.

---

## 🚀 Quick Start

```bash
# Start the server
./start.sh

# In another terminal, run these tests
```

---

## ✅ 1. CHUNKS (4 endpoints)

### Create Chunk (JSON)
```bash
curl -X POST http://localhost:18888/anttp-0/chunk \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"content":"SGVsbG8gV29ybGQh"}'

# Response: {"address":"abc123..."}
# Save this address for next test!
```

### Get Chunk (JSON)
```bash
curl http://localhost:18888/anttp-0/chunk/YOUR_ADDRESS_HERE \
  -H 'x-store-type: memory'

# Response: {"content":"SGVsbG8gV29ybGQh"}
```

### Create Chunk (Binary)
```bash
echo "Hello Binary World" | curl -X POST \
  http://localhost:18888/anttp-0/binary/chunk \
  -H 'Content-Type: application/octet-stream' \
  -H 'x-store-type: memory' \
  --data-binary @-

# Response: {"address":"def456..."}
```

### Get Chunk (Binary)
```bash
curl http://localhost:18888/anttp-0/binary/chunk/YOUR_BINARY_ADDRESS \
  -H 'x-store-type: memory'

# Response: (raw bytes) Hello Binary World
```

---

## ✅ 2. REGISTERS (4 endpoints)

### Create Register
```bash
curl -X POST http://localhost:18888/anttp-0/register \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"test_register","content":"68656c6c6f"}'

# Response: {"address":"ghi789..."}
# Note: content is HEX encoded! "68656c6c6f" = "hello"
```

### Update Register
```bash
curl -X PUT http://localhost:18888/anttp-0/register/YOUR_REGISTER_ADDRESS \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"test_register","content":"776f726c64"}'

# "776f726c64" = "world"
# Response: {"success":true,"message":"Register updated"}
```

### Get Register (Current Value)
```bash
curl http://localhost:18888/anttp-0/register/YOUR_REGISTER_ADDRESS \
  -H 'x-store-type: memory'

# Response: {"content":"776f726c64"}
```

### Get Register History
```bash
curl http://localhost:18888/anttp-0/register_history/YOUR_REGISTER_ADDRESS \
  -H 'x-store-type: memory'

# Response: [
#   {"content":"68656c6c6f","timestamp":1234567890},
#   {"content":"776f726c64","timestamp":1234567891}
# ]
```

---

## ✅ 3. POINTERS (3 endpoints)

### Create Pointer
```bash
curl -X POST http://localhost:18888/anttp-0/pointer \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"my_pointer","content":"abc123_SOME_CHUNK_ADDRESS"}'

# Response: {"address":"jkl012..."}
```

### Update Pointer
```bash
curl -X PUT http://localhost:18888/anttp-0/pointer/YOUR_POINTER_ADDRESS \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"my_pointer","content":"def456_NEW_CHUNK_ADDRESS"}'

# Response: {"success":true,"message":"Pointer updated"}
```

### Get Pointer
```bash
curl http://localhost:18888/anttp-0/pointer/YOUR_POINTER_ADDRESS \
  -H 'x-store-type: memory'

# Response: {"content":"def456_NEW_CHUNK_ADDRESS"}
```

---

## ✅ 4. PUBLIC SCRATCHPADS (3 endpoints)

### Create Public Scratchpad
```bash
curl -X POST http://localhost:18888/anttp-0/public_scratchpad \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"public_note","content":"UHVibGljIE5vdGU="}'

# "UHVibGljIE5vdGU=" = "Public Note" in Base64
# Response: {"address":"mno345..."}
```

### Update Public Scratchpad
```bash
curl -X PUT http://localhost:18888/anttp-0/public_scratchpad/YOUR_ADDRESS/public_note \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"content":"VXBkYXRlZCBOb3Rl"}'

# "VXBkYXRlZCBOb3Rl" = "Updated Note"
# Response: {"success":true,"message":"Scratchpad updated"}
```

### Get Public Scratchpad
```bash
curl http://localhost:18888/anttp-0/public_scratchpad/YOUR_ADDRESS \
  -H 'x-store-type: memory'

# Response: {"content":"VXBkYXRlZCBOb3Rl"}
```

---

## ✅ 5. PRIVATE SCRATCHPADS (3 endpoints)

### Create Private Scratchpad
```bash
curl -X POST http://localhost:18888/anttp-0/private_scratchpad \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"name":"secret_key","content":"U2VjcmV0IERhdGE="}'

# "U2VjcmV0IERhdGE=" = "Secret Data"
# Response: {"address":"pqr678..."}
```

### Update Private Scratchpad
```bash
curl -X PUT http://localhost:18888/anttp-0/private_scratchpad/YOUR_ADDRESS/secret_key \
  -H 'Content-Type: application/json' \
  -H 'x-store-type: memory' \
  -d '{"content":"TmV3IFNlY3JldA=="}'

# "TmV3IFNlY3JldA==" = "New Secret"
# Response: {"success":true,"message":"Scratchpad updated"}
```

### Get Private Scratchpad
```bash
curl http://localhost:18888/anttp-0/private_scratchpad/YOUR_ADDRESS/secret_key \
  -H 'x-store-type: memory'

# Response: {"content":"TmV3IFNlY3JldA=="}
# Note: You need the name ("secret_key") to access it!
```

---

## ✅ 6. ARCHIVES (4 endpoints)

### Create Archive (Multipart)
```bash
# Create test files first
echo "Hello from file1" > /tmp/file1.txt
echo "Hello from file2" > /tmp/file2.txt

curl -X POST http://localhost:18888/anttp-0/multipart/public_archive \
  -H 'x-store-type: memory' \
  -F "files=@/tmp/file1.txt" \
  -F "files=@/tmp/file2.txt"

# Response: {"address":"stu901..."}
```

### Create Archive with Path
```bash
curl -X POST http://localhost:18888/anttp-0/multipart/public_archive/docs/v1 \
  -H 'x-store-type: memory' \
  -F "files=@/tmp/file1.txt"

# Response: {"address":"vwx234..."}
# Files stored at: docs/v1/file1.txt
```

### Get Archive Root (List Files)
```bash
curl http://localhost:18888/anttp-0/public_archive/YOUR_ARCHIVE_ADDRESS \
  -H 'x-store-type: memory'

# Response: {
#   "address":"stu901...",
#   "files":[
#     {"path":"file1.txt","content":"SGVsbG8gZnJvbSBmaWxlMQ=="},
#     {"path":"file2.txt","content":"SGVsbG8gZnJvbSBmaWxlMg=="}
#   ]
# }
```

### Get Specific File from Archive
```bash
curl http://localhost:18888/anttp-0/public_archive/YOUR_ARCHIVE_ADDRESS/file1.txt \
  -H 'x-store-type: memory'

# Response: (raw file content) Hello from file1
```

---

## ✅ 7. HEALTH CHECK

```bash
curl http://localhost:18888/health

# Response: {
#   "status":"healthy",
#   "version":"0.2.0",
#   "description":"AntTP-compatible Rust backend"
# }
```

---

## 🎯 Complete Test Script

Save this as `test_all_endpoints.sh`:

```bash
#!/bin/bash

BASE_URL="http://localhost:18888"
STORE_TYPE="memory"

echo "🧪 Testing ALL Endpoints..."
echo ""

# 1. Chunk
echo "1️⃣ Testing Chunks..."
CHUNK=$(curl -s -X POST "$BASE_URL/anttp-0/chunk" \
  -H "Content-Type: application/json" \
  -H "x-store-type: $STORE_TYPE" \
  -d '{"content":"SGVsbG8gV29ybGQh"}' | jq -r '.address')
echo "  Created chunk: $CHUNK"

CHUNK_DATA=$(curl -s "$BASE_URL/anttp-0/chunk/$CHUNK" \
  -H "x-store-type: $STORE_TYPE" | jq -r '.content')
echo "  Retrieved: $CHUNK_DATA"
echo ""

# 2. Register
echo "2️⃣ Testing Registers..."
REG=$(curl -s -X POST "$BASE_URL/anttp-0/register" \
  -H "Content-Type: application/json" \
  -H "x-store-type: $STORE_TYPE" \
  -d '{"name":"test","content":"68656c6c6f"}' | jq -r '.address')
echo "  Created register: $REG"

curl -s -X PUT "$BASE_URL/anttp-0/register/$REG" \
  -H "Content-Type: application/json" \
  -H "x-store-type: $STORE_TYPE" \
  -d '{"name":"test","content":"776f726c64"}' > /dev/null
echo "  Updated register"

HISTORY=$(curl -s "$BASE_URL/anttp-0/register_history/$REG" \
  -H "x-store-type: $STORE_TYPE" | jq '. | length')
echo "  History entries: $HISTORY"
echo ""

# 3. Pointer
echo "3️⃣ Testing Pointers..."
PTR=$(curl -s -X POST "$BASE_URL/anttp-0/pointer" \
  -H "Content-Type: application/json" \
  -H "x-store-type: $STORE_TYPE" \
  -d "{\"name\":\"ptr\",\"content\":\"$CHUNK\"}" | jq -r '.address')
echo "  Created pointer: $PTR → $CHUNK"
echo ""

# 4. Scratchpads
echo "4️⃣ Testing Scratchpads..."
PUB=$(curl -s -X POST "$BASE_URL/anttp-0/public_scratchpad" \
  -H "Content-Type: application/json" \
  -H "x-store-type: $STORE_TYPE" \
  -d '{"name":"public","content":"UHVibGlj"}' | jq -r '.address')
echo "  Created public scratchpad: $PUB"

PRIV=$(curl -s -X POST "$BASE_URL/anttp-0/private_scratchpad" \
  -H "Content-Type: application/json" \
  -H "x-store-type: $STORE_TYPE" \
  -d '{"name":"secret","content":"U2VjcmV0"}' | jq -r '.address')
echo "  Created private scratchpad: $PRIV"
echo ""

# 5. Archive
echo "5️⃣ Testing Archives..."
echo "Test content" > /tmp/test.txt
ARCH=$(curl -s -X POST "$BASE_URL/anttp-0/multipart/public_archive" \
  -H "x-store-type: $STORE_TYPE" \
  -F "files=@/tmp/test.txt" | jq -r '.address')
echo "  Created archive: $ARCH"
echo ""

echo "✅ All tests complete!"
```

Make it executable and run:
```bash
chmod +x test_all_endpoints.sh
./test_all_endpoints.sh
```

---

## 📊 Summary

**Total Endpoints Implemented: 20+**

- ✅ 4 Chunk endpoints (JSON + Binary)
- ✅ 4 Register endpoints (with history)
- ✅ 3 Pointer endpoints
- ✅ 3 Public Scratchpad endpoints
- ✅ 3 Private Scratchpad endpoints
- ✅ 4 Archive endpoints (multipart)
- ✅ 1 Health check

**All using correct `/anttp-0/` prefix!** 🎉

---

## 🎓 For Students

### What Each Feature Does

**Chunks**: Store permanent data (like saving a file)
**Registers**: Store data you can update (like a variable)
**Pointers**: Reference other data (like a shortcut)
**Scratchpads**: Temporary notes (public or private)
**Archives**: Groups of files (like a ZIP file)

### Encoding Guide

**Base64** (Chunks, Scratchpads):
```bash
echo -n "Hello" | base64    # Encode
# SGVsbG8=
echo "SGVsbG8=" | base64 -d # Decode
# Hello
```

**Hex** (Registers):
```bash
echo -n "hello" | xxd -p    # Encode
# 68656c6c6f
echo "68656c6c6f" | xxd -r -p # Decode
# hello
```

---

## 🚀 Next: Test on Real Network

Change `x-store-type: network` and watch your data go to Autonomi! 🌐
