# AntTP Tutorial - Deployment Guide

## Quick Start (Docker - Recommended)

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+

### Steps

1. **Extract the archive:**
```bash
tar -xzf anttp-tutorial.tar.gz
cd anttp-tutorial
```

2. **Run the startup script:**
```bash
./start.sh
```

That's it! The application will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080

## Manual Setup (Without Docker)

### Backend Setup

1. **Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. **Build and run backend:**
```bash
cd backend
cargo build --release
./target/release/anttp-tutorial-backend
```

The backend will start on http://0.0.0.0:8080

### Frontend Setup

1. **Install Node.js 20+:**
```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Or download from: https://nodejs.org/
```

2. **Build and run frontend:**
```bash
cd frontend
npm install
npm run build
npm run preview
```

The frontend will start on http://localhost:3000

## VPS Deployment

### 1. Using Docker on VPS

```bash
# Connect to your VPS
ssh user@your-vps-ip

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo apt-get install docker-compose-plugin

# Upload your application
scp anttp-tutorial.tar.gz user@your-vps-ip:~/
ssh user@your-vps-ip
tar -xzf anttp-tutorial.tar.gz
cd anttp-tutorial

# Start the application
sudo docker-compose up -d
```

### 2. Setting up Reverse Proxy (Nginx)

Create `/etc/nginx/sites-available/anttp-tutorial`:

```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Frontend
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # Backend API
    location /api {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

Enable the site:
```bash
sudo ln -s /etc/nginx/sites-available/anttp-tutorial /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 3. SSL with Let's Encrypt

```bash
sudo apt-get install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

### 4. Environment Variables

Create `.env` file in project root:

```env
# Backend
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080
RUST_LOG=info

# Frontend
BACKEND_URL=http://backend:8080
```

Update `docker-compose.yml` to use it:

```yaml
services:
  backend:
    env_file: .env
  frontend:
    env_file: .env
```

## Production Considerations

### Security
- [ ] Use HTTPS (Let's Encrypt)
- [ ] Set up firewall (UFW)
- [ ] Enable CORS restrictions
- [ ] Add authentication if needed
- [ ] Regular security updates

### Monitoring
```bash
# View logs
docker-compose logs -f

# Check resource usage
docker stats

# Health check
curl http://localhost:8080/health
```

### Backup
```bash
# Backup docker volumes
docker-compose down
tar -czf backup-$(date +%Y%m%d).tar.gz docker_volumes/

# Restore
tar -xzf backup-20240101.tar.gz
docker-compose up -d
```

### Updates
```bash
# Pull latest changes
git pull

# Rebuild and restart
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

## Troubleshooting

### Services won't start
```bash
# Check logs
docker-compose logs backend
docker-compose logs frontend

# Verify ports
sudo lsof -i :8080
sudo lsof -i :3000

# Rebuild from scratch
docker-compose down -v
docker-compose build --no-cache
docker-compose up -d
```

### Frontend can't reach backend
1. Check `BACKEND_URL` in docker-compose.yml
2. Verify backend is running: `curl http://localhost:8080/health`
3. Check Docker network: `docker network inspect anttp-tutorial_default`

### Permission issues
```bash
# Fix file permissions
sudo chown -R $USER:$USER .
```

### Out of disk space
```bash
# Clean up Docker
docker system prune -a
docker volume prune
```

## Performance Tuning

### For production load:

1. **Increase backend workers** (in `backend/src/main.rs`):
```rust
HttpServer::new(...)
    .workers(4)  // Adjust based on CPU cores
    .bind(&bind_addr)?
```

2. **Enable production optimizations** in `backend/Cargo.toml`:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

3. **Frontend build optimization**:
```bash
cd frontend
npm run build -- --optimize
```

## System Requirements

### Minimum
- 1 CPU core
- 1GB RAM
- 5GB disk space

### Recommended
- 2+ CPU cores
- 2GB+ RAM
- 10GB disk space

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review application logs
3. Consult the main README.md
