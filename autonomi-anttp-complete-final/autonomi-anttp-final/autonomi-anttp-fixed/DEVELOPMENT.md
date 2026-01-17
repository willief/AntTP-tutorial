# Development Guide - Autonomi AntTP Explorer

This guide explains how to develop and test this TDD-based application.

## Quick Start

### 1. Start All Services

```bash
# From project root
docker-compose up -d
```

Wait for all services to start:
- AntTP: http://localhost:18888
- Backend API: http://localhost:8000
- Frontend: http://localhost:5173

### 2. Verify Services

```bash
# Check AntTP
curl http://localhost:18888/swagger-ui/

# Check Backend API
curl http://localhost:8000/health

# Check Frontend
open http://localhost:5173
```

## TDD Workflow

### Backend (Python + pytest)

#### 1. Write Test First

Create a test file in `backend/tests/unit/`:

```python
# backend/tests/unit/test_new_feature.py
import pytest

class TestNewFeature:
    @pytest.mark.asyncio
    async def test_feature_works(self):
        """Test that new feature works."""
        # Arrange
        service = NewService()
        
        # Act
        result = await service.do_something()
        
        # Assert
        assert result == expected_value
```

#### 2. Run Test (Should Fail)

```bash
cd backend
python -m pytest tests/unit/test_new_feature.py -v
```

Expected output: **FAILED** (feature not implemented yet)

#### 3. Implement Minimal Code

```python
# backend/app/services/new_service.py
class NewService:
    async def do_something(self):
        return expected_value  # Minimal implementation
```

#### 4. Run Test (Should Pass)

```bash
python -m pytest tests/unit/test_new_feature.py -v
```

Expected output: **PASSED**

#### 5. Refactor with Confidence

Now refactor the code knowing tests will catch regressions.

### Running All Backend Tests

```bash
cd backend

# Run all tests
python -m pytest tests/ -v

# Run with coverage
python -m pytest tests/ --cov=app --cov-report=html

# Run specific test file
python -m pytest tests/unit/test_chunks.py -v

# Run specific test function
python -m pytest tests/unit/test_chunks.py::TestChunkService::test_create_chunk_success -v

# Watch mode (install pytest-watch first)
pip install pytest-watch
ptw tests/
```

### Frontend (SvelteKit + Vitest)

#### 1. Write Test First

Create a test file in `frontend/tests/unit/`:

```typescript
// frontend/tests/unit/NewComponent.test.ts
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import NewComponent from '$lib/components/NewComponent.svelte';

describe('NewComponent', () => {
  it('should render correctly', () => {
    const { getByText } = render(NewComponent);
    expect(getByText('Expected Text')).toBeInTheDocument();
  });
});
```

#### 2. Run Test (Should Fail)

```bash
cd frontend
npm test
```

Expected output: **FAIL** (component not implemented yet)

#### 3. Implement Minimal Code

```svelte
<!-- frontend/src/lib/components/NewComponent.svelte -->
<script lang="ts">
  // Minimal implementation
</script>

<div>
  <p>Expected Text</p>
</div>
```

#### 4. Run Test (Should Pass)

```bash
npm test
```

Expected output: **PASS**

#### 5. Refactor with Confidence

Refactor knowing tests will catch issues.

### Running All Frontend Tests

```bash
cd frontend

# Run all tests
npm test

# Run with UI
npm run test:ui

# Run with coverage
npm run test:coverage

# Watch mode (runs automatically)
npm test -- --watch

# Run specific test file
npm test -- tests/unit/ChunkCreator.test.ts
```

## Integration Testing

### Backend Integration Tests

Integration tests hit real endpoints:

```python
# backend/tests/integration/test_chunks_api.py
import pytest
from httpx import AsyncClient
from app.main import app

class TestChunksAPI:
    @pytest.mark.asyncio
    async def test_create_chunk_endpoint(self):
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.post(
                "/api/v1/chunks",
                json={"content": "dGVzdA==", "store_type": "memory"}
            )
            assert response.status_code == 200
```

Run integration tests:

```bash
cd backend
python -m pytest tests/integration/ -v
```

### End-to-End Testing

Test the full stack with all services running:

```bash
# Ensure services are running
docker-compose up -d

# Run backend integration tests against live AntTP
cd backend
ANTTP_BASE_URL=http://localhost:18888 python -m pytest tests/integration/ -v

# Test frontend against live backend
cd frontend
VITE_API_BASE_URL=http://localhost:8000/api/v1 npm run test:integration
```

## Debugging

### Backend Debugging

```bash
# View backend logs
docker-compose logs -f backend

# Access backend container
docker-compose exec backend bash

# Run Python debugger
# Add breakpoint() in code, then:
docker-compose exec backend python -m pdb app/main.py
```

### Frontend Debugging

```bash
# View frontend logs
docker-compose logs -f frontend

# Access frontend container
docker-compose exec frontend sh

# Browser DevTools
# Open http://localhost:5173 and use browser debugger
```

### AntTP Debugging

```bash
# View AntTP logs
docker-compose logs -f anttp

# Check AntTP Swagger UI
open http://localhost:18888/swagger-ui/
```

## Code Quality

### Backend

```bash
cd backend

# Format with black
black app/ tests/

# Lint with ruff
ruff check app/ tests/

# Type check with mypy
mypy app/
```

### Frontend

```bash
cd frontend

# Type check
npm run check

# Lint
npm run lint
```

## Test Coverage Goals

- **Backend**: >85% coverage
- **Frontend**: >80% coverage

Check coverage:

```bash
# Backend
cd backend
python -m pytest tests/ --cov=app --cov-report=term-missing

# Frontend
cd frontend
npm run test:coverage
```

## Common Issues

### AntTP Not Starting

```bash
# Check AntTP container
docker-compose logs anttp

# Restart AntTP
docker-compose restart anttp
```

### Backend Can't Connect to AntTP

```bash
# Check network connectivity
docker-compose exec backend curl http://anttp:18888/health

# Verify ANTTP_BASE_URL
docker-compose exec backend env | grep ANTTP
```

### Frontend Can't Connect to Backend

```bash
# Check backend is running
curl http://localhost:8000/health

# Check CORS settings in backend
docker-compose logs backend | grep CORS
```

## Adding New Features (TDD Checklist)

- [ ] Write unit tests first
- [ ] Run tests (they should fail)
- [ ] Implement minimal code
- [ ] Run tests (they should pass)
- [ ] Write integration tests
- [ ] Run integration tests
- [ ] Refactor if needed
- [ ] Check test coverage
- [ ] Update documentation

## Useful Commands

```bash
# Rebuild containers after dependency changes
docker-compose build

# View all logs
docker-compose logs -f

# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v

# Scale services
docker-compose up -d --scale backend=2
```
