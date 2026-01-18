#!/bin/bash
# test_connectivity.sh

echo "Testing connectivity to Autonomi bootstrap servers..."
echo "====================================================="

BOOTSTRAP_SERVERS=(
  "159.89.251.80"
  "159.65.210.89" 
  "159.223.246.45"
  "139.59.201.153"
  "139.59.200.27"
  "13.228.91.146"
  "18.141.161.225"
)

for server in "${BOOTSTRAP_SERVERS[@]}"; do
  echo -n "Testing $server... "
  if timeout 5 curl -s -I "http://$server/bootstrap_cache.json" > /dev/null 2>&1; then
    echo "✓ REACHABLE"
  else
    echo "✗ UNREACHABLE"
  fi
done

echo ""
echo "Checking from inside Docker container..."
docker-compose exec anttp sh -c "apk add curl && curl -v http://159.89.251.80/bootstrap_cache.json" || echo "Failed to exec"