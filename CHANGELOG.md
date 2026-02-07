# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Phase 2: Real Autonomi Network integration (see ROADMAP.md)
- Video tutorials
- Dark mode support
- Enhanced error messages
- Loading states and animations

## [0.1.0] - 2026-02-03

### Added

**Backend (Rust/Actix-Web)**
- Complete REST API for all 6 Autonomi primitives
- Chunks: Immutable content-addressed storage
- Files: Large data with DataMaps and metadata
- Registers: Versioned key-value storage with CRDT simulation
- Pointers: Mutable references with counter tracking
- Archives: File collections with directory structure
- PNR: Personal Name Resolution with record types
- Health check endpoint
- CORS support for frontend
- In-memory storage with Mutex protection
- Comprehensive error handling

**Frontend (SvelteKit)**
- Homepage with all 6 primitives overview
- Individual pages for each primitive
- Sidebar navigation (desktop + mobile hamburger menu)
- Responsive design (mobile, tablet, desktop)
- Example buttons for quick testing
- Real-time form validation
- Clean, modern UI with Tailwind-inspired styling
- Detail views for created items
- Empty states

**DevOps**
- Docker Compose configuration
- Multi-stage Dockerfile for backend (Rust)
- Optimized Dockerfile for frontend (Node/Svelte)
- Rebuild script for clean builds
- Start script for quick deployment
- API test suite (30+ tests)

**Documentation**
- Comprehensive README with educational disclaimer
- Complete API reference (API.md)
- Quick start guide (QUICK-START.md)
- Deployment guide (DEPLOYMENT.md)
- Command reference (COMMANDS.md)
- Troubleshooting guide (TROUBLESHOOTING.md)
- Phase 2 roadmap (ROADMAP.md)
- Contributing guidelines (CONTRIBUTING.md)
- Git initialization guide (GIT_INIT_GUIDE.md)
- MIT License

### Features by Primitive

**Chunks**
- Store immutable content
- Content-addressed generation
- Size tracking
- List all chunks

**Files**
- Upload with name, content, content_type
- DataMap generation
- Metadata preservation (size, created_at)
- Content type selection dropdown
- File details view

**Registers**
- Key-value storage
- Version tracking
- Merkle register array simulation
- Update with version increment
- History visualization

**Pointers**
- Named mutable references
- Target updates with counter increment
- Owner tracking
- Version counter display
- Update interface

**Archives**
- Multiple file bundling
- Directory path preservation
- Custom metadata (author, version, description)
- Website example template
- Photo album example template
- File size calculation

**PNR**
- Human-readable name mapping
- Record type specification (chunk, file, register, pointer, archive)
- Name resolution
- Example mappings
- Type badges

### Technical Details
- Rust 1.93+
- Actix-Web 4.4
- SvelteKit 1.30
- Docker multi-stage builds
- Port configuration: 3000 (frontend), 8080 (backend)

### Documentation
- Clear educational disclaimer throughout
- Network integration roadmap
- Contribution guidelines
- API examples for all endpoints
- Testing instructions

### Known Limitations (Educational Mock)
- In-memory storage only (resets on restart)
- No real network connection
- No actual encryption
- Mock XOR addresses
- No payment integration
- No persistence

[Unreleased]: https://github.com/YOUR_USERNAME/anttp-tutorial/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR_USERNAME/anttp-tutorial/releases/tag/v0.1.0
