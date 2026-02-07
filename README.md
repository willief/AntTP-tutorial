# AntTP Tutorial - Interactive Learning Platform

An interactive web application for learning the Autonomi Network storage primitives through hands-on examples and a REST API.

> **⚠️ Educational Tutorial Notice**
> 
> This is a **tutorial/simulator** application designed for learning the Autonomi Network concepts. It currently uses **in-memory mock storage** and does not connect to the real Autonomi Network. Data is reset when the application restarts.
> 
> For production use with real network storage, see the [Network Integration Roadmap](ROADMAP.md).

## 🎯 What You'll Learn

This tutorial teaches the **six core storage primitives** of the Autonomi Network:

- **📦 Chunks** - Immutable, content-addressed storage (foundation for everything)
- **📁 Files** - Large data with automatic chunking and DataMaps
- **📊 Registers** - Versioned mutable key-value storage with CRDT
- **👉 Pointers** - Mutable references to other data
- **📚 Archives** - File collections for websites and applications
- **🏷️ PNR** - Personal Name Resolution (DNS-like naming)

## ✨ Features

- ✅ **Interactive UI** - Create and manage all 6 primitives through web forms
- ✅ **REST API** - Complete backend API for all storage types
- ✅ **Sidebar Navigation** - Easy access to all primitives
- ✅ **Responsive Design** - Works on desktop and mobile
- ✅ **Example Data** - Pre-filled examples for quick learning
- ✅ **API Documentation** - Complete endpoint reference
- ✅ **Test Suite** - Comprehensive API testing script

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose
- 2GB free RAM
- Ports 3000 (frontend) and 8080 (backend) available

### Installation

```bash
# Extract the archive
tar -xzf anttp-tutorial.tar.gz
cd anttp-tutorial

# Start everything
./rebuild.sh

# Or manually
docker compose up -d
```

### Access

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Health Check**: http://localhost:8080/health

## 📚 Usage Examples

### Web Interface

1. Navigate to http://localhost:3000
2. Click on any primitive in the sidebar
3. Try the example buttons to populate forms
4. Create, view, and manage your data

### API Examples

```bash
# Test all endpoints
./test-all-api.sh

# Create a chunk
curl -X POST http://localhost:8080/api/chunks \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello, Autonomi!"}'

# Upload a file
curl -X POST http://localhost:8080/api/files \
  -H "Content-Type: application/json" \
  -d '{"name":"document.txt","content":"File content here"}'

# Create a PNR mapping
curl -X POST http://localhost:8080/api/pnr \
  -H "Content-Type: application/json" \
  -d '{"name":"my-website","target":"archive_abc123","record_type":"archive"}'
```

See [API.md](API.md) for complete endpoint documentation.

## 🏗️ Architecture

### Stack

- **Backend**: Rust + Actix-Web + In-memory storage
- **Frontend**: SvelteKit + Tailwind-inspired CSS
- **Deployment**: Docker Compose

### Project Structure

```
anttp-tutorial/
├── backend/
│   ├── src/main.rs          # API implementation
│   ├── Cargo.toml           # Rust dependencies
│   └── Dockerfile
├── frontend/
│   ├── src/
│   │   ├── routes/          # Page components
│   │   ├── app.css          # Global styles
│   │   └── +layout.svelte   # Layout with sidebar
│   ├── package.json
│   └── Dockerfile
├── docker-compose.yml
├── API.md                   # Complete API docs
├── ROADMAP.md              # Network integration plan
├── rebuild.sh              # Clean rebuild script
└── test-all-api.sh         # API test suite
```

## 🧪 Testing

```bash
# Run complete API test suite (tests all 6 primitives)
./test-all-api.sh

# Test individual endpoints
curl http://localhost:8080/health
curl http://localhost:8080/api/chunks
```

Expected output: `Passed: 30+, Failed: 0`

## 📖 Learning Path

### Beginner

1. Start with **Chunks** - The foundation
2. Try **Files** - See how large data is handled
3. Explore **PNR** - Human-readable names

### Intermediate

4. Learn **Registers** - Mutable key-value storage
5. Try **Pointers** - Dynamic references
6. Combine primitives (PNR → Pointer → Archive)

### Advanced

7. Study **Archives** - Website hosting
8. Build complete workflow: Archive + Pointer + PNR
9. Read the API docs and build your own client

## 🔄 Real Network Integration

**This tutorial currently uses mock storage.** To connect to the real Autonomi Network:

1. See [ROADMAP.md](ROADMAP.md) for integration plan
2. Check the `network-integration` branch (coming soon)
3. Reference the [AntTP project](https://github.com/traktion/AntTP) for production implementation

### Key Differences

| Tutorial (Current) | Real Network |
|-------------------|--------------|
| In-memory storage | Distributed network storage |
| Instant operations | Network latency |
| No costs | Payment required (EVM tokens) |
| Resets on restart | Permanent storage |
| Mock addresses | Real XOR addresses |
| No encryption | Self-encryption |

## 🛠️ Development

### Run in Development Mode

```bash
# Backend
cd backend
cargo run

# Frontend
cd frontend
npm install
npm run dev
```

### Rebuild from Scratch

```bash
./rebuild.sh
```

This script:
- Stops all containers
- Removes old images
- Builds fresh (no cache)
- Shows logs for verification

## 📝 API Reference

All API endpoints follow REST conventions:

- **Chunks**: `/api/chunks`
- **Files**: `/api/files`
- **Registers**: `/api/registers`
- **Pointers**: `/api/pointers`
- **Archives**: `/api/archives`
- **PNR**: `/api/pnr`

See [API.md](API.md) for complete documentation with examples.

## 🤝 Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).

Areas for improvement:
- Network integration (see ROADMAP.md)
- Additional examples
- UI enhancements
- Documentation improvements
- Test coverage

## 📄 License

MIT License - See LICENSE file for details

## 🔗 Resources

- **Autonomi Network**: https://autonomi.com
- **Autonomi Docs**: https://docs.autonomi.com
- **AntTP (Production)**: https://github.com/traktion/AntTP
- **Autonomi SDK**: https://github.com/maidsafe/autonomi
- **CLI Tools**: https://github.com/maidsafe/autonomi/tree/main/ant-cli

## 🙏 Acknowledgments

- Built to demonstrate Autonomi Network concepts
- Based on patterns from the AntTP project
- Uses the Autonomi primitive architecture

## ⚠️ Disclaimer

This is an educational tool for learning purposes. For production applications:
- Use the official Autonomi SDK
- Connect to real network
- Implement proper error handling
- Add security measures
- Handle payments correctly

---

**Ready to learn Autonomi?** Start the tutorial and explore the primitives!

```bash
./rebuild.sh
open http://localhost:3000
```
