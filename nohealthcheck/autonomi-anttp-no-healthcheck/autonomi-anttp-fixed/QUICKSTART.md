# 🚀 Quick Start Guide

## Prerequisites
- Docker Desktop installed and running
- 4GB+ RAM available for Docker
- Ports 5173, 8000, 18888 available

## Installation

### 1. Extract the Archive
```bash
tar -xzf autonomi-anttp-complete.tar.gz
cd autonomi-anttp-fixed
```

### 2. Start Services
```bash
./start.sh
```

Or manually:
```bash
docker compose up -d
```

### 3. Wait for Initialization
⏱️ **IMPORTANT: AntTP takes 30-60 seconds to fully start!**

The containers start immediately, but AntTP needs time to initialize.

**Wait and watch the logs**:
```bash
docker compose logs -f anttp
```

**Look for this message** (means AntTP is ready):
```
Listening on http://0.0.0.0:18888
```

Or simply **wait 60 seconds** after starting, then try accessing the services.

### 4. Access the Application

Once running, open in your browser:
- **Frontend**: http://localhost:5173
- **Backend API Docs**: http://localhost:8000/docs
- **AntTP**: http://localhost:18888

## First Steps

1. **Explore the UI**: Visit http://localhost:5173
2. **Try Creating a Chunk**:
   - Go to http://localhost:5173/chunks
   - Enter some text
   - Select "Memory (Cache Only)" storage
   - Click "Create Chunk"
   - You'll get back an address!

3. **Check the API**:
   - Visit http://localhost:8000/docs
   - Try the POST `/api/v1/chunks` endpoint
   - Use this content (base64): `SGVsbG8gQXV0b25vbWk=`

## Running Tests

### Backend Tests
```bash
cd backend
python -m pytest tests/ -v
```

Expected: **13 tests pass** ✅

### Frontend Tests
```bash
cd frontend
npm install  # First time only
npm test
```

Expected: **11 tests pass** ✅

## Common Issues

### Services Won't Start
```bash
# Check logs
docker compose logs

# Restart everything
docker compose down
docker compose up -d
```

### Port Already in Use
```bash
# Change ports in docker-compose.yml
# Or stop conflicting services
```

### AntTP Taking Too Long
This is normal! AntTP initialization can take 60+ seconds.

Check progress:
```bash
docker compose logs -f anttp
```

Look for: `Listening on http://0.0.0.0:18888`

## What to Explore

### 📚 Documentation
- `README.md` - Project overview and architecture
- `TUTORIAL.md` - Complete AntTP features guide (6000+ words)
- `DEVELOPMENT.md` - TDD workflow and development guide
- `TROUBLESHOOTING.md` - Solutions to common problems

### 🧪 Test-Driven Development
All code was written **tests-first**! Check out:
- `backend/tests/unit/test_chunks.py` - Unit tests
- `backend/app/services/chunks.py` - Implementation
- `frontend/tests/unit/ChunkCreator.test.ts` - Component tests

### 🎯 Features Implemented
- ✅ **Chunks** - Immutable storage (fully tested)
- ✅ **Scratchpads** - Mutable data (fully tested)
- 🔨 **Registers** - Template ready for you to implement with TDD
- 🔨 **Archives** - Template ready
- 🔨 **Pointers** - Template ready
- 🔨 **PNR** - Template ready
- 🔨 **Graph** - Template ready

## Next Steps

1. **Read TUTORIAL.md** - Learn all AntTP features
2. **Study the TDD approach** - See how tests drive development
3. **Implement more features** - Follow TDD to add Registers, Archives, etc.
4. **Explore Autonomi** - This is decentralized storage!

## Stop Services

```bash
docker compose down
```

To also remove volumes:
```bash
docker compose down -v
```

## Need Help?

- Check `TROUBLESHOOTING.md`
- View logs: `docker compose logs -f`
- GitHub Issues (if this is public)
- [Autonomi Forum](https://forum.autonomi.community/)

---

**Happy coding with TDD! 🎉**
