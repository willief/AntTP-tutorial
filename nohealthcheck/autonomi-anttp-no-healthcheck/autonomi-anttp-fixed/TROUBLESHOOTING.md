# Troubleshooting Guide

## Common Issues and Solutions

### AntTP Container Health Check Failing

**Symptom**: `dependency failed to start: container anttp is unhealthy`

**Root Cause**: The AntTP Docker image doesn't include `wget` or `curl`, so health checks fail.

**Solution**: This version **removes health checks entirely**. Services start in order but don't wait for health. This is the correct approach.

**Important**: After starting, **wait 30-60 seconds** for AntTP to initialize before the backend/frontend can connect.

**Alternative - Start services manually with delays**:
   ```bash
   # Start AntTP first and wait
   docker compose up -d anttp
   sleep 30
   
   # Then start backend and frontend
   docker compose up -d backend frontend
   ```

3. **Check AntTP logs**:
   ```bash
   docker compose logs -f anttp
   ```
   
   Look for: `Listening on http://0.0.0.0:18888`

### Port Already in Use

**Symptom**: `port is already allocated`

**Solution**:
```bash
# Find what's using the port
lsof -i :18888  # or :8000 or :5173

# Kill the process or change ports in docker-compose.yml
```

### Backend Can't Connect to AntTP

**Symptom**: Backend returns 503 errors or "AntTP error"

**Solutions**:

1. **Check AntTP is running**:
   ```bash
   curl http://localhost:18888
   ```

2. **Check Docker network**:
   ```bash
   docker compose exec backend ping anttp
   ```

3. **Restart backend**:
   ```bash
   docker compose restart backend
   ```

### Frontend Can't Load

**Symptom**: White screen or "Failed to fetch"

**Solutions**:

1. **Check backend is running**:
   ```bash
   curl http://localhost:8000/health
   ```

2. **Check browser console** for CORS errors

3. **Clear browser cache** and reload

4. **Check environment variables**:
   ```bash
   docker compose exec frontend env | grep VITE
   ```

### Node Modules Issues (Frontend)

**Symptom**: `Cannot find module` or dependency errors

**Solution**:
```bash
# Rebuild frontend container
docker compose build --no-cache frontend
docker compose up -d frontend
```

### Python Dependencies Issues (Backend)

**Symptom**: `ModuleNotFoundError` or import errors

**Solution**:
```bash
# Rebuild backend container  
docker compose build --no-cache backend
docker compose up -d backend
```

### Database/Volume Issues

**Symptom**: Data not persisting or corrupted cache

**Solution**:
```bash
# Remove all volumes and restart
docker compose down -v
docker compose up -d
```

### Tests Failing

**Backend Tests**:
```bash
cd backend

# Install dependencies
pip install -r requirements.txt

# Run tests with verbose output
python -m pytest tests/ -v --tb=short

# Run specific test
python -m pytest tests/unit/test_chunks.py::TestChunkService::test_create_chunk_success -v
```

**Frontend Tests**:
```bash
cd frontend

# Install dependencies
npm install

# Run tests
npm test

# Run specific test
npm test -- tests/unit/ChunkCreator.test.ts
```

### Docker Compose Version Issues

**Symptom**: `the attribute 'version' is obsolete` warning

**Not a Problem**: This is just a warning in newer Docker Compose versions. You can:
- Ignore it (doesn't affect functionality)
- Or remove the `version: '3.8'` line from docker-compose.yml

### Slow Performance

**Solutions**:

1. **Increase Docker resources**:
   - Docker Desktop → Settings → Resources
   - Increase CPU and Memory

2. **Use memory storage for testing**:
   ```json
   {
     "store_type": "memory"
   }
   ```

3. **Check AntTP cache settings** in docker-compose.yml

## Debug Commands

### View All Logs
```bash
docker compose logs -f
```

### View Specific Service Logs
```bash
docker compose logs -f anttp
docker compose logs -f backend
docker compose logs -f frontend
```

### Access Container Shell
```bash
# Backend
docker compose exec backend bash

# Frontend
docker compose exec frontend sh

# AntTP (if needed)
docker compose exec anttp sh
```

### Check Service Status
```bash
docker compose ps
```

### Inspect Network
```bash
docker network inspect autonomi-anttp-explorer_autonomi-net
```

### Check Container Health
```bash
docker inspect anttp | grep -A 10 Health
```

## Reset Everything

If all else fails, nuclear option:

```bash
# Stop and remove everything
docker compose down -v

# Remove images
docker rmi autonomi-anttp-explorer-backend autonomi-anttp-explorer-frontend

# Rebuild from scratch
docker compose build --no-cache
docker compose up -d
```

## Getting Help

If you're still stuck:

1. **Check logs**: `docker compose logs`
2. **GitHub Issues**: Report bugs with logs attached
3. **Forum**: [Autonomi Forum](https://forum.autonomi.community/)
4. **Discord**: Join the Autonomi Discord

## Known Limitations

1. **AntTP Initialization**: Takes 30-60 seconds to fully start
2. **Network Storage**: Requires AntTP to be connected to Autonomi Network (alpha)
3. **Health Checks**: May report unhealthy during startup (use `service_started` condition)
4. **Hot Reload**: Frontend hot reload works, backend requires restart for some changes

## Performance Tips

1. **Use cache-only storage** for development:
   - Set `store_type: "memory"` or `"disk"`
   - Avoids network upload costs

2. **Increase timeouts** if on slow network:
   ```python
   # backend/app/config.py
   anttp_timeout: int = 60  # Increase from 30
   ```

3. **Disable uploads** for read-only testing:
   ```yaml
   # docker-compose.yml
   command: >
     /usr/local/bin/anttp
     --uploads-disabled
   ```

## Additional Resources

- [AntTP Documentation](https://github.com/traktion/AntTP)
- [FastAPI Docs](https://fastapi.tiangolo.com/)
- [SvelteKit Docs](https://kit.svelte.dev/)
- [Docker Compose Docs](https://docs.docker.com/compose/)
