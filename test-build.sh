#!/bin/bash

# Manual build and test script

echo "🔨 Building backend manually..."
cd backend

echo ""
echo "1️⃣ Building with cargo..."
cargo build --release 2>&1 | tail -20

echo ""
echo "2️⃣ Checking if binary exists..."
if [ -f target/release/anttp-tutorial-backend ]; then
    echo "✅ Binary found!"
    ls -lh target/release/anttp-tutorial-backend
    echo ""
    echo "3️⃣ Binary info:"
    file target/release/anttp-tutorial-backend
    echo ""
    echo "4️⃣ Testing binary..."
    timeout 3 target/release/anttp-tutorial-backend &
    sleep 2
    echo ""
    echo "5️⃣ Testing API..."
    curl -s http://localhost:8080/health || echo "❌ Backend not responding"
    killall anttp-tutorial-backend 2>/dev/null
else
    echo "❌ Binary NOT found!"
    echo "Build failed. Check errors above."
fi

cd ..
