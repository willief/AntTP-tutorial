#!/bin/bash
# stop.sh - Stop the AntTP backend server

echo "🛑 Stopping AntTP backend server..."

# Find and kill any process on port 18888
PID=$(lsof -ti:18888 2>/dev/null)

if [ -z "$PID" ]; then
    echo "ℹ️  No server running on port 18888"
else
    echo "   Found process: $PID"
    kill $PID 2>/dev/null
    sleep 1
    
    # Check if still running
    if lsof -ti:18888 > /dev/null 2>&1; then
        echo "   Force killing..."
        kill -9 $PID 2>/dev/null
    fi
    
    echo "✅ Server stopped"
fi

# Also check for any anttp-backend processes
PROCS=$(pgrep -f "anttp-backend" 2>/dev/null)
if [ ! -z "$PROCS" ]; then
    echo "   Cleaning up anttp-backend processes: $PROCS"
    pkill -f "anttp-backend"
    echo "✅ Processes cleaned up"
fi

echo ""
echo "🎯 Ready to start fresh!"
echo "   Run: ./start.sh"
