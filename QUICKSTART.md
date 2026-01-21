# ⚡ Quick Start - Autonomi Rust Backend

## For Complete Beginners

### 1. Install Prerequisites

**On macOS/Linux:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Docker Desktop
# Download from: https://www.docker.com/products/docker-desktop
```

**On Windows:**
```powershell
# Install Rust
# Download from: https://rustup.rs/

# Install Docker Desktop
# Download from: https://www.docker.com/products/docker-desktop
```

### 2. Start the Project

```bash
cd autonomi-rust-backend
./start.sh
```

Wait 60 seconds for AntTP to initialize, then visit:
- **Frontend**: http://localhost:5173
- **Backend**: http://localhost:8000/health

### 3. Run Tests

```bash
./test.sh
```

## Common Commands

### Development

```bash
# Check code compiles
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Build release
cargo build --release

# Run locally (needs AntTP running)
cargo run
```

### Testing

```bash
# All unit tests
cargo test --lib

# Specific test
cargo test test_validate_base64

# With output
cargo test -- --nocapture

# Integration tests (needs AntTP)
cargo test -- --ignored
```

### Docker

```bash
# Start all services
docker compose up -d

# View logs
docker compose logs -f

# Restart backend
docker compose restart backend

# Rebuild after changes
docker compose up -d --build

# Stop everything
docker compose down

# Reset volumes
docker compose down -v
```

## Test Chunk Creation (cURL)

```bash
# Create a chunk
curl -X POST http://localhost:8000/chunks \
  -H "Content-Type: application/json" \
  -d '{
    "content": "SGVsbG8sIFJ1c3Qh",
    "storage_type": "network"
  }'

# Get a chunk (replace {address} with actual address)
curl http://localhost:8000/chunks/{address}
```

## Project Structure at a Glance

```
src/
├── main.rs              ← Start here! Entry point
├── config.rs            ← Configuration
├── models/
│   └── mod.rs           ← Data types (like Pydantic)
├── services/
│   └── chunk_service.rs ← Business logic
└── handlers/
    └── chunk_handler.rs ← HTTP endpoints
```

## TDD Workflow

**When adding a new feature:**

1. **RED** - Write failing test
```rust
#[tokio::test]
async fn test_new_feature() {
    let result = service.new_feature().await;
    assert!(result.is_ok());  // ❌ FAILS initially
}
```

2. **GREEN** - Make it pass
```rust
pub async fn new_feature(&self) -> Result<(), Error> {
    // Minimal implementation
    Ok(())  // ✅ PASSES now
}
```

3. **REFACTOR** - Improve code
```rust
pub async fn new_feature(&self) -> Result<(), Error> {
    // Better implementation
    // Add logging, error handling, etc.
    Ok(())  // ✅ Still PASSES
}
```

## Debugging

### Backend not starting?

```bash
# Check logs
docker compose logs backend

# Common issues:
# - AntTP not ready (wait 60s)
# - Port 8000 in use (lsof -i :8000)
# - Build failed (cargo build)
```

### Tests failing?

```bash
# Run single test with output
cargo test test_name -- --nocapture

# Common issues:
# - AntTP not running (for integration tests)
# - Code doesn't compile (cargo check)
# - Wrong test command (see README)
```

### Docker issues?

```bash
# Rebuild completely
docker compose down
docker compose build --no-cache
docker compose up -d

# Clean everything
docker system prune -a
```

## Where to Get Help

1. **README.md** - Full documentation
2. **Code comments** - Every function documented
3. **Tests** - Examples of how to use code
4. **Cargo errors** - Usually very helpful!

## Next Steps

1. ✅ Get it running (`./start.sh`)
2. ✅ Run tests (`./test.sh`)
3. 📚 Read `README.md` for details
4. 🧪 Study tests to understand TDD
5. 🚀 Add new features following TDD

**You're ready to go!** 🦀
