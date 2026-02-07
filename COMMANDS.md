# AntTP Tutorial - Quick Commands

## Starting Fresh

```bash
# Clean rebuild (removes everything and rebuilds)
./rebuild.sh
```

## Daily Use

```bash
# Start services
docker compose up -d

# Stop services
docker compose down

# View logs (live)
docker compose logs -f

# View logs (backend only)
docker compose logs -f backend

# Check status
docker compose ps
```

## Troubleshooting

```bash
# Backend keeps restarting?
docker compose logs backend

# Frontend not updating?
docker compose build --no-cache frontend
docker compose up -d

# Complete reset
docker compose down -v
docker rmi anttp-tutorial-backend anttp-tutorial-frontend
docker compose up -d --build
```

## File Permissions Issue?

If you extracted as root and files are owned by root:

```bash
# Fix ownership (replace 'willie' with your username)
sudo chown -R willie:willie anttp-tutorial/

# Or if you know your UID (usually 1000)
sudo chown -R $(id -u):$(id -g) anttp-tutorial/
```

## Accessing the Application

- **Frontend**: http://localhost:3000
- **Backend Health**: http://localhost:8080/health
- **Backend API**: http://localhost:8080/api/*

## Testing the API

```bash
# Health check
curl http://localhost:8080/health

# Create a chunk
curl -X POST http://localhost:8080/api/chunks \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello AntTP!"}'

# List chunks
curl http://localhost:8080/api/chunks
```

## Port Conflicts?

If ports 3000 or 8080 are in use:

```bash
# Check what's using the ports
lsof -i :3000
lsof -i :8080

# Change ports in docker-compose.yml
# Edit the 'ports' section to use different ports
```
