#!/bin/bash
# start-frontend.sh - Start the SvelteKit frontend

echo "🎨 ════════════════════════════════════════════════════════"
echo "🎨  AntTP Frontend - Quick Start"
echo "🎨 ════════════════════════════════════════════════════════"

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install Node.js first:"
    echo "   https://nodejs.org/"
    exit 1
fi

echo "✅ Node.js found: $(node --version)"

# Navigate to frontend directory
cd "$(dirname "$0")/frontend" || exit 1

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo ""
    echo "📦 Installing dependencies (first time only)..."
    npm install
    
    if [ $? -ne 0 ]; then
        echo "❌ Failed to install dependencies"
        exit 1
    fi
fi

# Create .env if it doesn't exist
if [ ! -f ".env" ]; then
    echo ""
    echo "📝 Creating .env file..."
    cp .env.example .env
fi

echo ""
echo "🚀 Starting development server..."
echo "   Frontend will be available at: http://localhost:5173"
echo "   Backend should be running at: http://localhost:18888"
echo ""
echo "Press Ctrl+C to stop"
echo ""
echo "🎨 ════════════════════════════════════════════════════════"
echo ""

# Start the dev server
npm run dev
