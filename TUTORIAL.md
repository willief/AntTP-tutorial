# AntTP Features Tutorial

This tutorial guides you through all AntTP features using the Autonomi AntTP Explorer application.

## Table of Contents

1. [Introduction](#introduction)
2. [Chunks](#chunks)
3. [Scratchpads](#scratchpads)
4. [Registers](#registers)
5. [Archives & Files](#archives--files)
6. [Pointers](#pointers)
7. [PNR (Pointer Name Resolution)](#pnr-pointer-name-resolution)
8. [Graph Entries](#graph-entries)
9. [System Commands](#system-commands)

---

## Introduction

AntTP is an HTTP server that makes the Autonomi Network accessible through standard HTTP requests. This means you can interact with a decentralized, immutable data network using familiar web technologies.

### Storage Types

All AntTP operations support three storage types:

- **Memory**: Data cached in RAM only (volatile, fastest, for testing)
- **Disk**: Data cached on disk (volatile, persists across restarts)
- **Network**: Data uploaded to Autonomi Network (permanent, decentralized)

---

## Chunks

**Chunks are immutable data blocks** - the fundamental unit of storage on Autonomi.

### Key Concepts

- **Content-Addressed**: Address (XOR) is derived from content hash
- **Immutable**: Cannot be changed once created
- **Deduplication**: Same content = same address
- **Permanent**: Once on network, always accessible

### Creating a Chunk

**Via UI:**

1. Navigate to `/chunks`
2. Enter text content
3. Select storage type
4. Click "Create Chunk"
5. Copy the returned address

**Via API:**

```bash
# Base64 encode your content
echo -n "Hello Autonomi" | base64  # SGVsbG8gQXV0b25vbWk=

# Create chunk (memory storage)
curl -X POST http://localhost:8000/api/v1/chunks \
  -H "Content-Type: application/json" \
  -d '{
    "content": "SGVsbG8gQXV0b25vbWk=",
    "store_type": "memory"
  }'

# Response:
# {
#   "address": "71b9fcd6d0fff9da53d2833ebc8d795527d28dfbcb90cee118be25ca57a63873",
#   "store_type": "memory"
# }
```

### Retrieving a Chunk

**Via API:**
```bash
# Get chunk by address
curl http://localhost:8000/api/v1/chunks/71b9fcd6d0fff9da53d2833ebc8d795527d28dfbcb90cee118be25ca57a63873

# Decode content
echo "SGVsbG8gQXV0b25vbWk=" | base64 -d  # Hello Autonomi
```

**Via AntTP Directly:**
```bash
# Access via AntTP
curl http://localhost:18888/71b9fcd6d0fff9da53d2833ebc8d795527d28dfbcb90cee118be25ca57a63873
```

### Use Cases

- **Content Distribution**: Share files via their addresses
- **Data Integrity**: Verify content hasn't changed
- **Deduplication**: Save storage by reusing addresses
- **Permanent URLs**: Addresses never change

---

## Scratchpads

**Scratchpads are mutable data stores** - temporary, editable storage.

### Types

1. **Public Scratchpads**: Unencrypted, anyone can read
2. **Private Scratchpads**: Encrypted with a name/key

### Creating a Public Scratchpad

**Via API:**
```bash
# Create public scratchpad
curl -X POST http://localhost:8000/api/v1/scratchpads/public \
  -H "Content-Type: application/json" \
  -d '{
    "content": "This is my public note",
    "store_type": "memory"
  }'

# Response:
# {
#   "address": "a33082163be512fb471a1cca385332b32c19917deec3989a97e100d827f97baf",
#   "is_private": false,
#   "store_type": "memory"
# }
```

### Updating a Scratchpad

```bash
# Update the scratchpad
curl -X PUT http://localhost:8000/api/v1/scratchpads/public/a33082... \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Updated content",
    "store_type": "memory"
  }'
```

### Creating a Private Scratchpad

```bash
# Create private scratchpad
curl -X POST http://localhost:8000/api/v1/scratchpads/private \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Secret information",
    "name": "my-secret-pad",
    "store_type": "memory"
  }'
```

### Retrieving a Private Scratchpad

```bash
# Must provide the name to decrypt
curl "http://localhost:8000/api/v1/scratchpads/private/{address}?name=my-secret-pad"
```

### Use Cases

- **Temporary Notes**: Quick notes that can be updated
- **Configuration**: Mutable app settings
- **User Preferences**: Per-user editable data
- **Collaborative Editing**: Shared mutable documents

---

## Registers

**Registers are signed, versioned mutable data** - with full history.

### Key Features

- **Signed**: Cryptographically signed updates
- **Versioned**: Full history of all changes
- **Auditable**: Can verify who made changes and when

### Creating a Register

```bash
# Create register (hex-encoded content)
curl -X POST http://localhost:8000/api/v1/registers \
  -H "Content-Type: application/json" \
  -d '{
    "content": "48656c6c6f",
    "name": "my-register",
    "store_type": "memory"
  }'
```

### Updating a Register

```bash
# Update register
curl -X PUT http://localhost:8000/api/v1/registers/{address} \
  -H "Content-Type: application/json" \
  -d '{
    "content": "576f726c64",
    "store_type": "memory"
  }'
```

### Viewing Register History

```bash
# Get full history
curl http://localhost:8000/api/v1/registers/{address}/history

# Response shows all versions:
# {
#   "address": "...",
#   "history": [
#     {"content": "48656c6c6f", "timestamp": "2025-01-15T10:00:00Z"},
#     {"content": "576f726c64", "timestamp": "2025-01-15T10:05:00Z"}
#   ]
# }
```

### Use Cases

- **Audit Logs**: Track all changes to data
- **Version Control**: Keep history of document edits
- **Smart Contracts**: Signed, verifiable transactions
- **Access Control**: Track who accessed what

---

## Archives & Files

**Archives are collections of files** - like directories.

### Creating an Archive

```bash
# Create archive with multiple files
curl -X POST http://localhost:8000/api/v1/archives/public \
  -H "Content-Type: application/json" \
  -d '{
    "files": [
      {"name": "index.html", "content": "PGh0bWw+..."},
      {"name": "style.css", "content": "Ym9keXs..."},
      {"name": "app.js", "content": "Y29uc3Q..."}
    ],
    "store_type": "network"
  }'
```

### Accessing Archive Files

```bash
# Via AntTP - list files
curl http://localhost:18888/{archive_address}/

# Access specific file
curl http://localhost:18888/{archive_address}/index.html
```

### Uploading Single Files

```bash
# Upload single file as public data
curl -X POST http://localhost:8000/api/v1/data/public \
  -H "Content-Type: application/json" \
  -d '{
    "content": "SGVsbG8gV29ybGQ=",
    "mime_type": "text/plain",
    "store_type": "network"
  }'
```

### Use Cases

- **Static Websites**: Host entire websites
- **File Sharing**: Share multiple related files
- **Documentation**: Host docs with assets
- **Web Apps**: Deploy SPAs (React, Vue, Svelte)

---

## Pointers

**Pointers are mutable references** to network addresses - like DNS.

### Why Use Pointers?

Immutable data gets a new address when updated. Pointers let you:
- Keep a stable address
- Update where it points
- Create URL shortcuts

### Creating a Pointer

```bash
# Create pointer to a chunk
curl -X POST http://localhost:8000/api/v1/pointers \
  -H "Content-Type: application/json" \
  -d '{
    "target_address": "71b9fcd6d0fff9da...",
    "name": "my-document",
    "store_type": "network"
  }'

# Response:
# {
#   "address": "80fad1f709a2b5d9...",
#   "store_type": "network"
# }
```

### Updating a Pointer

```bash
# Point to a new target
curl -X PUT http://localhost:8000/api/v1/pointers/{pointer_address} \
  -H "Content-Type: application/json" \
  -d '{
    "target_address": "new_chunk_address_here...",
    "store_type": "network"
  }'
```

### Following a Pointer

```bash
# Resolves automatically via AntTP
curl http://localhost:18888/{pointer_address}/
```

### Use Cases

- **Version Management**: Update content, keep same URL
- **Redirects**: Point to different resources
- **Bookmarks**: Personal shortcuts to data
- **Dynamic Content**: Update what users see

---

## PNR (Pointer Name Resolution)

**PNR is like DNS for Autonomi** - map names to addresses.

### PNR Structure

```
Resolver Pointer → Personal Pointer → PNR Zone Chunk → Target
```

### Creating a PNR Zone

```bash
# Create PNR zone with name
curl -X POST http://localhost:8000/api/v1/pnr/zones \
  -H "Content-Type: application/json" \
  -d '{
    "name": "mywebsite",
    "default_record": {
      "sub_name": "",
      "address": "archive_address_here...",
      "record_type": "X",
      "ttl": 60
    },
    "store_type": "network"
  }'

# Response:
# {
#   "name": "mywebsite",
#   "personal_address": "...",
#   "resolver_address": "...",
#   "records": [...]
# }
```

### Accessing via PNR

```bash
# Via proxy
curl http://mywebsite/

# Via gateway
curl https://anttp.antsnest.site/mywebsite/

# With subdomain (future)
curl http://blog.mywebsite/
```

### Use Cases

- **Human-Readable URLs**: Use names instead of hashes
- **Website Hosting**: mysite.autonomi instead of long hash
- **Subdomains**: blog.mysite, api.mysite
- **Service Discovery**: Find services by name

---

## Graph Entries

**Graph entries are structured, linked data** - like a database.

### Creating Graph Entries

```bash
# Create graph node
curl -X POST http://localhost:8000/api/v1/graph/entries \
  -H "Content-Type: application/json" \
  -d '{
    "content": {
      "type": "person",
      "name": "Alice",
      "connections": ["bob_address", "carol_address"],
      "metadata": {"role": "developer"}
    },
    "store_type": "network"
  }'
```

### Retrieving Graph Entries

```bash
# Get graph node
curl http://localhost:8000/api/v1/graph/entries/{address}
```

### Use Cases

- **Social Networks**: User profiles and connections
- **Knowledge Graphs**: Linked information
- **Supply Chains**: Track product relationships
- **Organization Charts**: Hierarchical data

---

## System Commands

**Monitor background operations** via the command queue.

### Viewing Command Queue

```bash
# List recent commands
curl http://localhost:8000/api/v1/commands

# Response shows upload queue:
# {
#   "commands": [
#     {
#       "id": 1,
#       "command_name": "CreateChunkCommand",
#       "state": "running",
#       "running_at": 1705315200000
#     }
#   ],
#   "count": 1
# }
```

### Use Cases

- **Upload Status**: Check if data finished uploading
- **Queue Management**: See what's pending
- **Performance Monitoring**: Track operation times
- **Debugging**: Understand what AntTP is doing

---

## Complete Example Workflow

Here's a complete workflow using multiple features:

```bash
# 1. Create content as chunks
curl -X POST http://localhost:8000/api/v1/chunks \
  -d '{"content":"'$(echo -n "v1.0" | base64)'", "store_type":"network"}' \
  | jq -r '.address' > chunk_v1.txt

# 2. Create a pointer to the chunk
curl -X POST http://localhost:8000/api/v1/pointers \
  -d '{"target_address":"'$(cat chunk_v1.txt)'", "store_type":"network"}' \
  | jq -r '.address' > pointer.txt

# 3. Create PNR zone for the pointer
curl -X POST http://localhost:8000/api/v1/pnr/zones \
  -d '{
    "name":"myapp",
    "default_record":{
      "address":"'$(cat pointer.txt)'",
      "record_type":"X",
      "ttl":60
    },
    "store_type":"network"
  }'

# 4. Access via friendly name
curl http://localhost:18888/myapp/

# 5. Update content (new version)
curl -X POST http://localhost:8000/api/v1/chunks \
  -d '{"content":"'$(echo -n "v2.0" | base64)'", "store_type":"network"}' \
  | jq -r '.address' > chunk_v2.txt

# 6. Update pointer to new content
curl -X PUT http://localhost:18888/api/v1/pointers/$(cat pointer.txt) \
  -d '{"target_address":"'$(cat chunk_v2.txt)'"}'

# 7. Access again - now shows v2.0!
curl http://localhost:18888/myapp/
```

---

## Testing Locally

All features can be tested with `store_type: "memory"` to avoid network costs:

```bash
# Test chunk creation locally
curl -X POST http://localhost:8000/api/v1/chunks \
  -H "x-cache-only: true" \
  -d '{"content":"dGVzdA==", "store_type":"memory"}'
```

---

## Next Steps

1. **Explore the UI**: Visit http://localhost:5173
2. **Try Each Feature**: Follow tutorials above
3. **Build Something**: Create your own app
4. **Read the Code**: Check out the TDD test examples
5. **Contribute**: Add more features with TDD!

## Resources

- [AntTP GitHub](https://github.com/traktion/AntTP)
- [Autonomi Forum](https://forum.autonomi.community/)
- [API Documentation](http://localhost:8000/docs)
- [AntTP Swagger](http://localhost:18888/swagger-ui/)
