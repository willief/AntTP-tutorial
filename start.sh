#!/bin/bash

# AntTP Tutorial - Quick Start Script
# This script helps you get the application running quickly

set -e

echo "🚀 AntTP Tutorial Application - Quick Start"
echo "==========================================="
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    echo "   Visit: https://docs.docker.com/get-docker/"
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    echo "   Visit: https://docs.docker.com/compose/install/"
    exit 1
fi

echo "✅ Docker and Docker Compose are installed"
echo ""

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1 ; then
        echo "⚠️  Port $port is already in use"
        echo "   Please stop the service using this port or modify docker-compose.yml"
        return 1
    fi
    return 0
}

# Check required ports
echo "🔍 Checking if ports are available..."
if check_port 8080 && check_port 3000; then
    echo "✅ Ports 8080 and 3000 are available"
else
    exit 1
fi
echo ""

# Build and start containers
echo "🏗️  Building Docker images..."
docker-compose build

echo ""
echo "🚀 Starting services..."
docker-compose up -d

echo ""
echo "⏳ Waiting for services to be ready..."
sleep 5

# Check if services are running
if docker-compose ps | grep -q "Up"; then
    echo ""
    echo "✅ Application is running!"
    echo ""
    echo "📱 Access the application:"
    echo "   Frontend: http://localhost:3000"
    echo "   Backend:  http://localhost:8080"
    echo "   Health:   http://localhost:8080/health"
    echo ""
    echo "📝 Useful commands:"
    echo "   View logs:        docker-compose logs -f"
    echo "   Stop services:    docker-compose down"
    echo "   Restart services: docker-compose restart"
    echo ""
    echo "🎓 Start learning AntTP storage primitives!"
else
    echo ""
    echo "❌ Something went wrong. Check logs with: docker-compose logs"
    exit 1
fi
