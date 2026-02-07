# Troubleshooting Guide

## Backend Keeps Restarting

If you see `Restarting (0) Less than a second ago` for the backend:

### Step 1: Check the logs
```bash
docker compose logs backend
```

If you see **no output**, the binary isn't running at all.

### Step 2: Run diagnostics
```bash
./diagnose-backend.sh
```

### Step 3: Force clean rebuild
```bash
./rebuild.sh
```

### Common Causes

**1. Binary not built properly**
- The Dockerfile might have cached an incomplete build
- Solution: Clean rebuild with `./rebuild.sh`

**2. Architecture mismatch**
- If you're on ARM (like Apple Silicon) and Docker is using x86
- Solution: Make sure Docker Desktop is set to use native architecture

**3. Missing dependencies**
- The Debian slim image might be missing required libraries
- Check with: `docker compose exec backend ldd /app/anttp-tutorial-backend`

## Frontend Not Showing New Design

If you still see the old simple homepage:

```bash
# Rebuild just the frontend
docker compose build --no-cache frontend
docker compose up -d frontend

# Or full rebuild
./rebuild.sh
```

Clear your browser cache:
- Chrome/Edge: Ctrl+Shift+Delete
- Firefox: Ctrl+Shift+Delete
- Or just hard refresh: Ctrl+F5

## No Sidebar Visible

The sidebar should be visible on desktop (> 1024px width).

On mobile, look for the hamburger menu (☰) in the top-left.

If it's missing:
1. Check browser console for JavaScript errors (F12)
2. Verify the build completed: `docker compose logs frontend | grep "vite"`
3. Rebuild: `./rebuild.sh`

## Port Already in Use

```bash
# Find what's using the port
lsof -i :3000
lsof -i :8080

# Kill the process or change ports in docker-compose.yml
```

## Permission Issues

If you get "Permission denied" errors:

```bash
# Fix ownership
sudo chown -R $(id -u):$(id -g) anttp-tutorial/
```

## Docker Warnings

**"version is obsolete"**
- This is just a warning, can be ignored
- Already removed in latest version

**"FromAsCasing"**
- Just a style warning, doesn't affect functionality

## Complete Reset

If nothing works:

```bash
# Stop everything
docker compose down -v

# Remove images
docker rmi anttp-tutorial-backend anttp-tutorial-frontend

# Remove the directory
cd ..
rm -rf anttp-tutorial

# Start fresh
tar -xzf anttp-tutorial.tar.gz
cd anttp-tutorial
./rebuild.sh
```

## Getting Help

1. Run `./diagnose-backend.sh` and share the output
2. Check `docker compose logs backend` for errors
3. Verify Docker is working: `docker ps`
4. Check disk space: `df -h`
