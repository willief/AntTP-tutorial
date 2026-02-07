# AntTP Tutorial - Complete API Documentation

> **⚠️ Educational Mock Implementation**
>
> This API uses in-memory storage for educational purposes. Data is not persisted to the real Autonomi Network and will be lost on restart. For production use, see the [Network Integration Roadmap](ROADMAP.md).

## Overview

This tutorial backend implements all major Autonomi Network storage primitives:

1. **Chunks** - Immutable content-addressed storage
2. **Files** - Large data with automatic chunking
3. **Registers** - Versioned key-value storage with CRDT
4. **Pointers** - Mutable references to other data
5. **Archives** - Collections of files with metadata
6. **PNR** - Personal Name Resolution (human-readable names)

Base URL: `http://localhost:8080`

---

## 📦 Chunks API

**Immutable, content-addressed storage**

### Store Chunk
```bash
POST /api/chunks
Content-Type: application/json

{
  "content": "Your data here"
}

Response:
{
  "success": true,
  "address": "chunk_abc123",
  "size": 14,
  "message": "Chunk stored successfully"
}
```

### Get Chunk
```bash
GET /api/chunks/{address}

Response:
{
  "success": true,
  "chunk": {
    "content": "Your data here",
    "address": "chunk_abc123",
    "size": 14
  }
}
```

### List Chunks
```bash
GET /api/chunks

Response:
{
  "success": true,
  "count": 5,
  "chunks": [...]
}
```

---

## 📁 Files API

**Large data with chunking and metadata**

### Upload File
```bash
POST /api/files
Content-Type: application/json

{
  "name": "document.pdf",
  "content": "base64_encoded_content",
  "content_type": "application/pdf"  // optional
}

Response:
{
  "success": true,
  "data_map": "datamap_xyz789",
  "size": 1024,
  "message": "File uploaded successfully"
}
```

### Get File
```bash
GET /api/files/{data_map}

Response:
{
  "success": true,
  "file": {
    "name": "document.pdf",
    "data_map": "datamap_xyz789",
    "size": 1024,
    "content_type": "application/pdf",
    "created_at": "1633024800"
  }
}
```

### List Files
```bash
GET /api/files

Response:
{
  "success": true,
  "count": 10,
  "files": [...]
}
```

---

## 📊 Registers API

**Versioned key-value storage with conflict resolution**

### Set Register (Create or Update)
```bash
POST /api/registers
Content-Type: application/json

{
  "key": "user:alice:profile",
  "value": "{\"name\":\"Alice\",\"level\":42}"
}

Response (Create):
{
  "success": true,
  "register": {
    "key": "user:alice:profile",
    "value": "{\"name\":\"Alice\",\"level\":42}",
    "version": 1,
    "merkle_reg": ["v1:{...}"]
  },
  "message": "Register created successfully"
}

Response (Update):
{
  "success": true,
  "register": {
    "key": "user:alice:profile",
    "value": "{\"name\":\"Alice\",\"level\":43}",
    "version": 2,
    "merkle_reg": ["v1:{...}", "v2:{...}"]
  },
  "message": "Register updated successfully"
}
```

### Get Register
```bash
GET /api/registers/{key}

Response:
{
  "success": true,
  "register": {
    "key": "user:alice:profile",
    "value": "{\"name\":\"Alice\",\"level\":42}",
    "version": 1,
    "merkle_reg": [...]
  }
}
```

### List Registers
```bash
GET /api/registers

Response:
{
  "success": true,
  "count": 15,
  "registers": [...]
}
```

---

## 👉 Pointers API

**Mutable references to other data**

### Create Pointer
```bash
POST /api/pointers
Content-Type: application/json

{
  "name": "latest-version",
  "target": "chunk_abc123"
}

Response:
{
  "success": true,
  "pointer": {
    "name": "latest-version",
    "target": "chunk_abc123",
    "counter": 1,
    "owner": "demo_owner_key"
  },
  "message": "Pointer created successfully"
}
```

### Update Pointer
```bash
PUT /api/pointers/{name}
Content-Type: application/json

{
  "name": "latest-version",
  "target": "chunk_xyz789"
}

Response:
{
  "success": true,
  "pointer": {
    "name": "latest-version",
    "target": "chunk_xyz789",
    "counter": 2,
    "owner": "demo_owner_key"
  },
  "message": "Pointer updated successfully"
}
```

### Get Pointer
```bash
GET /api/pointers/{name}

Response:
{
  "success": true,
  "pointer": {
    "name": "latest-version",
    "target": "chunk_xyz789",
    "counter": 2,
    "owner": "demo_owner_key"
  }
}
```

### List Pointers
```bash
GET /api/pointers

Response:
{
  "success": true,
  "count": 8,
  "pointers": [...]
}
```

---

## 📚 Archives API

**Collections of files with metadata**

### Create Archive
```bash
POST /api/archives
Content-Type: application/json

{
  "name": "my-website",
  "files": [
    {
      "path": "index.html",
      "content": "<html>...</html>"
    },
    {
      "path": "css/style.css",
      "content": "body { ... }"
    }
  ],
  "metadata": {
    "author": "Alice",
    "version": "1.0",
    "description": "My personal website"
  }
}

Response:
{
  "success": true,
  "archive": {
    "name": "my-website",
    "address": "archive_def456",
    "files": [
      {
        "path": "index.html",
        "data_map": "datamap_abc",
        "size": 120
      },
      {
        "path": "css/style.css",
        "data_map": "datamap_xyz",
        "size": 450
      }
    ],
    "metadata": {
      "author": "Alice",
      "version": "1.0"
    }
  },
  "message": "Archive created successfully"
}
```

### Get Archive
```bash
GET /api/archives/{address}

Response:
{
  "success": true,
  "archive": {
    "name": "my-website",
    "address": "archive_def456",
    "files": [...],
    "metadata": {...}
  }
}
```

### List Archives
```bash
GET /api/archives

Response:
{
  "success": true,
  "count": 3,
  "archives": [...]
}
```

---

## 🏷️ PNR API

**Personal Name Resolution - Human-readable names**

### Create PNR Entry
```bash
POST /api/pnr
Content-Type: application/json

{
  "name": "my-website",
  "target": "archive_def456",
  "record_type": "archive"
}

Response:
{
  "success": true,
  "pnr": {
    "name": "my-website",
    "target": "archive_def456",
    "record_type": "archive"
  },
  "message": "PNR entry created successfully"
}
```

### Resolve PNR Name
```bash
GET /api/pnr/{name}

Response:
{
  "success": true,
  "pnr": {
    "name": "my-website",
    "target": "archive_def456",
    "record_type": "archive"
  }
}
```

### List PNR Entries
```bash
GET /api/pnr

Response:
{
  "success": true,
  "count": 12,
  "entries": [...]
}
```

---

## Error Responses

All endpoints return error responses in this format:

```json
{
  "success": false,
  "error": "Description of what went wrong"
}
```

Common HTTP status codes:
- `200` - Success
- `404` - Resource not found
- `400` - Bad request (invalid data)
- `500` - Server error

---

## Testing

Run the complete test suite:
```bash
./test-all-api.sh
```

This tests all endpoints for all 6 primitives.

---

## Use Cases by Primitive

### Chunks
- Store images, documents, media
- Archive immutable records
- Content-addressed assets

### Files
- Upload large files (automatically chunked)
- Store binary data
- Maintain file metadata

### Registers
- User profiles and settings
- Application configuration
- Game state with history

### Pointers
- "Latest version" references
- Mutable bookmarks
- Dynamic redirects

### Archives
- Website hosting
- Photo albums
- Document collections

### PNR
- Friendly names for all resources
- Easy-to-remember aliases
- Decentralized naming system

---

## Architecture Notes

This is a **tutorial/mock implementation** that demonstrates the API patterns. In a real Autonomi application:

1. **Chunks** would be stored using content-addressing (hash-based)
2. **Files** would be automatically split and encrypted
3. **Registers** would use actual CRDT conflict resolution
4. **Pointers** would require cryptographic signatures
5. **Archives** would leverage efficient storage deduplication
6. **PNR** would integrate with the network's naming system

All data here is stored in-memory and resets when the server restarts.
