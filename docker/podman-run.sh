#!/usr/bin/env bash
set -euo pipefail

# Podman run script equivalent to docker-compose.yml

NETWORK=ota-net
VOLUME=ota-pgdata
DB_IMAGE=logicpi/ota-database:0.5.2
BACKEND_IMAGE=logicpi/ota-backend:0.6.0
SERVER_IMAGE=logicpi/ota-server:0.4.2

echo "Ensuring Podman network and volume exist..."
if ! podman network inspect "$NETWORK" >/dev/null 2>&1; then
  podman network create "$NETWORK"
fi
podman volume inspect "$VOLUME" >/dev/null 2>&1 || podman volume create "$VOLUME"

echo "Starting ota-database..."
podman rm -f ota-database >/dev/null 2>&1 || true
podman run -d \
  --name ota-database \
  --network "$NETWORK" \
  -v "$VOLUME":/var/lib/postgresql/data \
  "$DB_IMAGE" \
  postgres -c "max_connections=1000"

echo "Waiting for database to accept connections (checking with pg_isready)..."
for i in {1..30}; do
  if podman exec ota-database pg_isready -U craftor >/dev/null 2>&1; then
    echo "Database ready"
    break
  fi
  sleep 2
done

echo "Starting ota-backend..."
podman rm -f ota-backend >/dev/null 2>&1 || true
podman run -d \
  --name ota-backend \
  --network "$NETWORK" \
  -p 20000:20000 \
  -e FW_DB='postgres://craftor:3.1415926@ota-database:5432/firmware' \
  "$BACKEND_IMAGE"

echo "Starting ota-server..."
podman rm -f ota-server >/dev/null 2>&1 || true
podman run -d \
  --name ota-server \
  --network "$NETWORK" \
  -p 9999:9999 \
  -e FW_SERVER='http://ota-backend:20000' \
  -e FW_DB='postgres://craftor:3.1415926@ota-database:5432/firmware' \
  "$SERVER_IMAGE"

echo "All services started. Use 'podman ps' to verify."
