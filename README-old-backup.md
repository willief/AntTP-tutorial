# AntTP Tutorial - Autonomi Storage Primitives

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.93%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com/)

> **Educational Tutorial Application** - Learn the six core storage primitives of the Autonomi Network through an interactive web interface.

## 🎓 What is This?

This is an **educational tutorial application** that teaches developers how to work with Autonomi Network storage primitives through:

- ✅ **Interactive Web UI** - Full-stack application with forms and examples
- ✅ **REST API** - Backend demonstrating API patterns for all 6 primitives
- ✅ **Working Examples** - Real-world use cases and code patterns
- ✅ **Complete Documentation** - API reference and guides

## ⚠️ Important: Educational Mock Implementation

**This tutorial currently uses in-memory mock storage for educational purposes.**

- 📚 **Purpose**: Teach concepts, API patterns, and use cases
- 🔄 **Data Persistence**: In-memory only (resets on restart)
- 🚫 **Network Integration**: Not connected to real Autonomi Network
- 🎯 **Target Audience**: Developers learning Autonomi primitives

**See [ROADMAP.md](ROADMAP.md) for plans to integrate with the real Autonomi Network (Phase 2).**

## 📦 The Six Storage Primitives

### 1. **Chunks** 📦
Immutable, content-addressed storage - the foundation of all data types.
- Store once, never changes
- XOR hash-based addressing
- Self-encryption
- Building block for everything else

### 2. **Files** 📁
Large data with automatic chunking and DataMaps.
- Automatic file splitting
- DataMap for reassembly
- Preserves metadata
- Efficient storage

### 3. **Registers** 📊
Versioned mutable key-value storage with CRDT conflict resolution.
- Key-value pairs
- Automatic versioning
- Merkle tree history
- Conflict resolution

### 4. **Pointers** 👉
Mutable references that can point to any data type.
- Dynamic target updates
- Version counter
- Ownership control
- Perfect for "latest" references

### 5. **Archives** 📚
File containers for websites and applications.
- Bundle multiple files
- Directory structure preservation
- Public or typed archives (tarchive)
- Website hosting

### 6. **PNR (Personal Name Resolution)** 🏷️
DNS-like naming system for the Autonomi Network.
- Human-readable names
- Maps to any primitive type
- Decentralized namespace
- Updatable mappings

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose
- 2GB free RAM
- Ports 3000 and 8080 available

### Installation

```bash
# Extract the archive
tar -xzf anttp-tutorial.tar.gz
cd anttp-tutorial

# Start the application
./rebuild.sh

# Or manually
docker compose up -d
```

### Access

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Health Check**: http://localhost:8080/health

## 📖 Using the Tutorial

### Interactive Web Interface

1. **Home Page** - Overview of all 6 primitives with comparisons
2. **Individual Pages** - Each primitive has its own interactive page:
   - Create/upload data
   - View examples
   - Test operations
   - See real responses

### Navigation

- **Desktop**: Persistent sidebar on the left
- **Mobile**: Hamburger menu (☰) 
- **Quick Links**: Backend health check in sidebar

### Testing the API

```bash
# Test all endpoints
./test-all-api.sh

# Manual API calls
curl http://localhost:8080/health
curl -X POST http://localhost:8080/api/chunks \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello, Autonomi!"}'
```

## 📚 Documentation

- **[API.md](API.md)** - Complete API reference with examples
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Production deployment guide
- **[QUICK-START.md](QUICK-START.md)** - Quick reference guide
- **[COMMANDS.md](COMMANDS.md)** - Common commands and troubleshooting
- **[ROADMAP.md](ROADMAP.md)** - Future network integration plans

## 🏗️ Architecture

### Backend (Rust/Actix-Web)

```
backend/
├── src/
│   └── main.rs          # API implementation (all 6 primitives)
├── Cargo.toml           # Dependencies
└── Dockerfile           # Multi-stage build
```

**Technologies:**
- Rust 1.93+
- Actix-Web 4.4
- In-memory storage (Mutex<Vec>)

### Frontend (SvelteKit)

```
frontend/
├── src/
│   ├── routes/          # Page components
│   │   ├── +page.svelte # Homepage
│   │   ├── chunks/      # Chunks page
│   │   ├── files/       # Files page
│   │   ├── registers/   # Registers page
│   │   ├── pointers/    # Pointers page
│   │   ├── archives/    # Archives page
│   │   └── pnr/         # PNR page
│   ├── app.css          # Global styles
│   └── +layout.svelte   # Layout with sidebar
└── package.json
```

**Technologies:**
- SvelteKit 1.30
- Svelte 4.2
- Vite 4.5
- Responsive design

## 🛠️ Development

### Project Structure

```
anttp-tutorial/
├── backend/             # Rust backend
├── frontend/            # SvelteKit frontend
├── docker-compose.yml   # Service orchestration
├── rebuild.sh           # Clean rebuild script
├── start.sh             # Quick start script
├── test-all-api.sh      # API test suite
└── docs/                # Documentation
```

### Local Development

```bash
# Backend
cd backend
cargo run

# Frontend
cd frontend
npm install
npm run dev

# Tests
cd backend
cargo test
```

### Docker Development

```bash
# Rebuild everything
./rebuild.sh

# View logs
docker compose logs -f

# Restart service
docker compose restart backend
```

## 🧪 Testing

### API Test Suite

```bash
./test-all-api.sh
```

Tests all 6 primitives with **30+ test cases**:
- Chunks: Store, retrieve, list
- Files: Upload, get, list  
- Registers: Create, update, versioning
- Pointers: Create, update, counter increment
- Archives: Create with multiple files
- PNR: Create mappings, resolve names

### Manual Testing

```bash
# Chunks
curl -X POST http://localhost:8080/api/chunks \
  -H "Content-Type: application/json" \
  -d '{"content":"Test data"}'

# Files
curl -X POST http://localhost:8080/api/files \
  -H "Content-Type: application/json" \
  -d '{"name":"test.txt","content":"File content"}'

# Registers
curl -X POST http://localhost:8080/api/registers \
  -H "Content-Type: application/json" \
  -d '{"key":"config","value":"{\"theme\":\"dark\"}"}'
```

## 🎯 Real-World Patterns

### Updatable Website with Friendly Name

Combine primitives for powerful patterns:

```bash
# 1. Create website archive
POST /api/archives
{
  "name": "my-site",
  "files": [
    {"path": "index.html", "content": "..."},
    {"path": "style.css", "content": "..."}
  ]
}
# Returns: archive_abc123...

# 2. Create pointer to archive
POST /api/pointers
{
  "name": "current-site",
  "target": "archive_abc123..."
}
# Returns: pointer with counter: 1

# 3. Create friendly name
POST /api/pnr
{
  "name": "mysite.antp",
  "target": "<pointer-address>",
  "record_type": "pointer"
}

# To update site: Create new archive, update pointer!
PUT /api/pointers/current-site
{
  "target": "archive_xyz789..."
}
# Counter increments to 2, site updates!
```

## 🔮 Future: Real Network Integration

**Phase 2** will integrate the actual Autonomi SDK:

- ✅ Real XOR network addressing
- ✅ Self-encryption of data
- ✅ Actual chunk splitting and DataMaps
- ✅ Data retrievable with CLI tools
- ✅ EVM-based payment integration
- ✅ Persistent storage on Autonomi Network

See [ROADMAP.md](ROADMAP.md) for detailed integration plans.

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Areas for contribution:**
- Bug fixes and improvements
- Additional examples and documentation
- UI/UX enhancements
- Network integration (Phase 2)
- Additional language bindings

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Autonomi Network** - For the revolutionary decentralized platform
- **AntTP Project** - Inspiration from the production HTTP server
- **MaidSafe** - For developing the Autonomi protocol
- **Community** - For feedback and contributions

## 📞 Support

- **Documentation**: https://docs.autonomi.com
- **Issues**: GitHub Issues
- **Community**: Autonomi Forums
- **AntTP Reference**: https://github.com/traktion/AntTP

## 🔗 Related Projects

- **Autonomi Network**: https://autonomi.com
- **AntTP Server**: https://github.com/traktion/AntTP
- **Autonomi CLI**: https://github.com/maidsafe/autonomi
- **Official Docs**: https://docs.autonomi.com

---

**Note**: This is an educational tutorial. For production applications, integrate with the real Autonomi Network SDK. See [ROADMAP.md](ROADMAP.md) for integration path.
