# Autonomi AntTP Explorer

A comprehensive web application exploring all features of AntTP (Autonomi HTTP Proxy) built with **Test-Driven Development (TDD)**.

## Architecture

- **Backend**: Python (FastAPI) - API wrapper around AntTP REST endpoints
- **Frontend**: SvelteKit - Interactive UI for exploring Autonomi Network features
- **Proxy**: AntTP (Docker) - HTTP server for Autonomi Network
- **Testing**: pytest (Python) + Vitest (SvelteKit)

## Features Explored

This application demonstrates all AntTP capabilities:

### 1. **Chunks** (Immutable Data Storage)

- Create chunks with base64-encoded content
- Retrieve chunks by address
- Explore content-addressed storage

### 2. **Scratchpads** (Volatile Mutable Data)

- Public scratchpads (unencrypted)
- Private scratchpads (encrypted)
- Create, update, and retrieve operations

### 3. **Registers** (Signed Mutable Data)

- Create registers with versioned content
- Update register values
- Retrieve current value and full history

### 4. **Archives & Files**

- Public archives (file collections)
- Single file uploads (public data)
- Directory listings and file browsing

### 5. **Graph Entries**

- Create structured graph nodes
- Retrieve graph data
- Explore relationships

### 6. **Pointers & PNR** (Pointer Name Resolution)

- Create pointers to network addresses
- Update pointer targets
- PNR zones with DNS-like functionality
- Subdomain support

### 7. **System Operations**

- Command queue management
- Background task monitoring

## Project Structure

```
autonomi-anttp-explorer/
├── backend/                 # Python FastAPI application
│   ├── app/
│   │   ├── __init__.py
│   │   ├── main.py         # FastAPI app entry point
│   │   ├── models/         # Pydantic models
│   │   ├── services/       # AntTP service wrappers
│   │   ├── routers/        # API endpoints
│   │   └── config.py       # Configuration
│   ├── tests/
│   │   ├── unit/           # Unit tests (TDD first)
│   │   ├── integration/    # Integration tests
│   │   └── conftest.py     # pytest fixtures
│   ├── requirements.txt
│   └── Dockerfile
├── frontend/               # SvelteKit application
│   ├── src/
│   │   ├── routes/         # SvelteKit routes
│   │   ├── lib/
│   │   │   ├── components/ # Svelte components
│   │   │   ├── stores/     # State management
│   │   │   └── api/        # API client
│   │   └── app.html
│   ├── tests/
│   │   ├── unit/           # Component unit tests
│   │   └── integration/    # Integration tests
│   ├── package.json
│   └── vite.config.ts
├── docker-compose.yml      # Multi-container orchestration
└── README.md
```

## TDD Approach

### 1. **Unit Tests First** (Core TDD)

- Write tests before implementation
- Test individual functions/components in isolation
- Fast feedback loop for refactoring
- **Python**: `pytest` with high coverage
- **SvelteKit**: `vitest` for component logic

### 2. **Integration Tests**

- Test API endpoint interactions with AntTP
- Test frontend-backend communication
- Validate data flow through the stack

### 3. **E2E Tests** (Critical Paths)

- User workflows for each AntTP feature
- Upload/download operations
- Pointer resolution chains

## Getting Started

### Prerequisites

- Docker & Docker Compose
- Python 3.11+
- Node.js 20+
- npm or pnpm

### Quick Start

1. **Clone and setup**:

```bash
git clone <repository>
cd autonomi-anttp-explorer
```

2. **Start all services**:

```bash
docker-compose up -d
```

This starts:

- AntTP on `http://localhost:18888`
- Backend API on `http://localhost:8000`
- Frontend on `http://localhost:5173`

3 **Run tests**:

```bash
# Backend tests
cd backend
python -m pytest tests/ -v

# Frontend tests  
cd frontend
npm test
```

## Development Workflow

### Helper Scripts

Three scripts are provided for easy management:

**`./start.sh`** - Quick Start

- Checks prerequisites  
- Creates Python venv for local testing
- Starts all Docker containers
- Waits for services (60s countdown)
- Shows access URLs and usage tips

**`./test.sh`** - Run All Tests

- Automatically creates Python venv
- Installs all dependencies
- Runs backend tests with pytest
- Runs frontend tests with vitest
- Shows coverage reports

**`./debug.sh`** - Troubleshooting

- Checks Docker status
- Shows container health
- Tests service endpoints
- Displays recent logs
- Shows environment config
- Lists useful debug commands

### Backend Development (TDD)

1. **Write test first** (`tests/unit/test_chunks.py`):

```python
def test_create_chunk():
    service = ChunkService()
    result = service.create_chunk("test data")
    assert result["address"] is not None
```

2 **Run test (should fail)**:

```bash
pytest tests/unit/test_chunks.py -v
```

3 **Implement minimal code** (`app/services/chunks.py`)
4. **Run test (should pass)**
5. **Refactor with confidence**

### Frontend Development (TDD)

1. **Write test first** (`tests/unit/ChunkCreator.test.ts`):

```typescript
test('creates chunk with valid data', async () => {
  const wrapper = mount(ChunkCreator);
  await wrapper.find('input').setValue('test');
  await wrapper.find('button').trigger('click');
  expect(wrapper.emitted('created')).toBeTruthy();
});
```

2 **Run test (should fail)**
3. **Implement component**
4. **Run test (should pass)**
5. **Refactor**

## API Documentation

Once running, visit:

- FastAPI Swagger: `http://localhost:8000/docs`
- AntTP Swagger: `http://localhost:18888/swagger-ui/`

## Testing Strategy

### Backend Tests

- **Unit**: Services, models, utilities (TDD core)
- **Integration**: API endpoints + AntTP interaction
- **Coverage target**: >85%

### Frontend Tests

- **Unit**: Component logic, stores, utilities
- **Integration**: API client, data flow
- **Coverage target**: >80%

## Contributing

1. Write tests first (TDD)
2. Implement minimal code to pass
3. Refactor
4. Ensure all tests pass
5. Submit PR with test coverage

## License

MIT

## Resources

- [AntTP Repository](https://github.com/traktion/AntTP)
- [Autonomi Forum Discussion](https://forum.autonomi.community/t/anttp-serving-autonomi-data-over-http/39718)
- [FastAPI Documentation](https://fastapi.tiangolo.com/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
