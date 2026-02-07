# AntTP Tutorial Application

A comprehensive tutorial application demonstrating the core storage primitives of the Autonomi Network Transfer Protocol (AntTP).

## Overview

This application provides an interactive learning environment for understanding and working with AntTP's four main storage primitives:

- **Chunks**: Immutable, content-addressed data storage
- **Scratchpads**: Mutable data with public/private access control
- **Registers**: Distributed key-value storage with versioning
- **PNR (Personal Name Resolution)**: Human-readable names for network addresses

## Architecture

### Backend (Rust/Actix-Web)
- RESTful API server on port 8080
- In-memory storage for demonstration purposes
- Full CRUD operations for all primitives
- JSON request/response format

### Frontend (SvelteKit)
- Modern responsive UI on port 3000
- Light blue homepage background
- Interactive forms for each primitive
- Real-time data display and management

## Prerequisites

- Docker and Docker Compose
- OR:
  - Rust 1.75+
  - Node.js 20+

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start the application
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the application
docker-compose down
```

The application will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080

### Manual Setup

#### Backend
```bash
cd backend
cargo build --release
cargo run
```

#### Frontend
```bash
cd frontend
npm install
npm run dev
```

## API Endpoints

### Health Check
- `GET /health` - Server health status

### Chunks
- `POST /api/chunks` - Store a new chunk
- `GET /api/chunks` - List all chunks
- `GET /api/chunks/{address}` - Get chunk by address

### Scratchpads
- `POST /api/scratchpads` - Create new scratchpad
- `GET /api/scratchpads` - List all scratchpads
- `GET /api/scratchpads/{name}` - Get scratchpad by name
- `PUT /api/scratchpads/{name}` - Update scratchpad

### Registers
- `POST /api/registers` - Set register value
- `GET /api/registers` - List all registers
- `GET /api/registers/{key}` - Get register by key

### PNR
- `POST /api/pnr` - Create PNR entry
- `GET /api/pnr` - List all PNR entries
- `GET /api/pnr/{name}` - Resolve name to address

## Project Structure

```
anttp-tutorial/
├── docker-compose.yml          # Docker orchestration
├── backend/
│   ├── Cargo.toml             # Rust dependencies
│   ├── Dockerfile             # Backend container
│   └── src/
│       └── main.rs            # API server implementation
├── frontend/
│   ├── package.json           # Node dependencies
│   ├── Dockerfile             # Frontend container
│   ├── svelte.config.js       # SvelteKit configuration
│   ├── vite.config.js         # Vite configuration
│   └── src/
│       ├── app.css            # Global styles
│       ├── routes/
│       │   ├── +layout.svelte # Main layout with navigation
│       │   ├── +page.svelte   # Homepage
│       │   ├── chunks/        # Chunks page
│       │   ├── scratchpads/   # Scratchpads page
│       │   ├── registers/     # Registers page
│       │   └── pnr/           # PNR page
│       └── lib/               # Shared components
└── README.md                  # This file
```

## Features

### Chunks Page
- Store immutable data
- View all stored chunks
- Display chunk content and addresses
- Learn about content-addressed storage

### Scratchpads Page
- Create mutable data containers
- Toggle public/private access
- Edit existing scratchpads
- Understand state management

### Registers Page
- Set key-value pairs
- Version tracking (increments on updates)
- Example templates for common use cases
- Learn about distributed storage

### PNR Page
- Create human-readable name mappings
- Resolve names to network addresses
- View all registered names
- Understand decentralized naming

## Development

### Backend Development
```bash
cd backend
cargo watch -x run  # Auto-reload on changes
```

### Frontend Development
```bash
cd frontend
npm run dev         # Development server with hot reload
```

### Building for Production
```bash
# Backend
cd backend
cargo build --release

# Frontend
cd frontend
npm run build
```

## Customization

### Styling
All styles are in `frontend/src/app.css`. Key CSS variables:

```css
:root {
  --primary-color: #2563eb;
  --home-bg: #add8e6;  /* Light blue homepage */
  /* ... other variables */
}
```

### Backend Configuration
Environment variables for the backend:
- `BACKEND_HOST` (default: 0.0.0.0)
- `BACKEND_PORT` (default: 8080)
- `RUST_LOG` (default: info)

### Frontend Configuration
Environment variables for the frontend:
- `BACKEND_URL` - Backend API URL

## Learning Path

1. **Start with Chunks** - Understanding immutable storage
2. **Move to Scratchpads** - Learning about mutable data
3. **Explore Registers** - Key-value storage with versioning
4. **Finish with PNR** - Human-readable addressing

## Troubleshooting

### Port Already in Use
```bash
# Check what's using the ports
sudo lsof -i :8080
sudo lsof -i :3000

# Change ports in docker-compose.yml if needed
```

### Docker Issues
```bash
# Rebuild containers
docker-compose build --no-cache

# Remove volumes and restart
docker-compose down -v
docker-compose up -d
```

### Backend Not Responding
```bash
# Check backend logs
docker-compose logs backend

# Test health endpoint
curl http://localhost:8080/health
```

### Frontend Not Loading
```bash
# Check frontend logs
docker-compose logs frontend

# Verify backend connection
# Check BACKEND_URL in docker-compose.yml
```

## Next Steps

This is a demonstration application. For production use with real Autonomi Network:

1. Replace in-memory storage with actual AntTP client
2. Add authentication and authorization
3. Implement proper error handling
4. Add data persistence
5. Set up monitoring and logging

## Resources

- [Autonomi Network Documentation](https://autonomi.com/docs)
- [AntTP Specification](https://github.com/maidsafe/antttp)
- [Rust Actix-Web Guide](https://actix.rs/)
- [SvelteKit Documentation](https://kit.svelte.dev/)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions welcome! Please open an issue or pull request.

## Author

Created as a tutorial application for learning AntTP storage primitives.
