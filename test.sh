#!/bin/bash

# Local Testing Script - Run tests without Docker

set -e

echo "🧪 Autonomi AntTP Explorer - Local Testing"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for Python
if ! command_exists python3; then
    echo "❌ Error: Python 3 is not installed"
    echo "Please install Python 3.11+ to run tests"
    exit 1
fi

# Check for Node
if ! command_exists node; then
    echo "❌ Error: Node.js is not installed"
    echo "Please install Node.js 20+ to run frontend tests"
    exit 1
fi

echo "✅ Prerequisites found"
echo ""

# Backend Tests
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐍 Backend Tests (Python + pytest)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cd backend

# Create venv if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
fi

# Activate venv
echo "Activating virtual environment..."
source venv/bin/activate

# Install/upgrade dependencies
echo "Installing dependencies..."
pip install --quiet --upgrade pip
pip install --quiet -r requirements.txt

echo ""
echo "Running backend tests..."
echo ""

# Run pytest with coverage
python -m pytest tests/ -v --cov=app --cov-report=term-missing

BACKEND_EXIT=$?

deactivate
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎨 Frontend Tests (Node + Vitest)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cd frontend

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "Installing npm dependencies..."
    npm install
    echo ""
fi

echo "Running frontend tests..."
echo ""

# Run vitest
npm test

FRONTEND_EXIT=$?

cd ..

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Test Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ $BACKEND_EXIT -eq 0 ]; then
    echo "✅ Backend tests: PASSED"
else
    echo "❌ Backend tests: FAILED"
fi

if [ $FRONTEND_EXIT -eq 0 ]; then
    echo "✅ Frontend tests: PASSED"
else
    echo "❌ Frontend tests: FAILED"
fi

echo ""

# Exit with error if any tests failed
if [ $BACKEND_EXIT -ne 0 ] || [ $FRONTEND_EXIT -ne 0 ]; then
    exit 1
fi

echo "🎉 All tests passed!"
echo ""
