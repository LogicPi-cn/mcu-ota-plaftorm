#!/usr/bin/env bash
set -euo pipefail

# Create a single Podman pod and run ota-database, ota-backend, ota-server inside it.
# Services will share network namespace so they can use localhost to talk to Postgres.

POD=ota-pod
VOLUME=pgdata
DB_IMAGE=docker.io/logicpi/ota-database:0.5.2
BACKEND_IMAGE=docker.io/logicpi/ota-backend:0.6.0
SERVER_IMAGE=docker.io/logicpi/ota-server:0.4.2

echo "Removing existing pod/container names if present (safe to ignore errors)"
podman pod rm -f "$POD" >/dev/null 2>&1 || true
podman rm -f ota-database ota-backend ota-server >/dev/null 2>&1 || true

echo "Creating pod '$POD' with host ports 20000 and 9999"
podman pod create --name "$POD" -p 20000:20000 -p 9999:9999

echo "Ensure volume exists: $VOLUME"
podman volume inspect "$VOLUME" >/dev/null 2>&1 || podman volume create "$VOLUME"

echo "Starting ota-database in pod"
podman run -d \
  --name ota-database \
  --pod "$POD" \
  -v "$VOLUME":/var/lib/postgresql/data \
  $DB_IMAGE \
  postgres -c "max_connections=1000"

echo "Waiting for database to accept connections..."
for i in {1..60}; do
  if podman exec ota-database pg_isready -U craftor >/dev/null 2>&1; then
    echo "Database ready"
    break
  fi
  sleep 2
done

echo "Starting ota-backend in pod (connecting to localhost:5432)"
podman run -d \
  --name ota-backend \
  --pod "$POD" \
  -e FW_DB='postgres://craftor:3.1415926@localhost:5432/firmware' \
  $BACKEND_IMAGE

echo "Starting ota-server in pod (backend at localhost:20000)"
podman run -d \
  --name ota-server \
  --pod "$POD" \
  -e FW_SERVER='http://localhost:20000' \
  -e FW_DB='postgres://craftor:3.1415926@localhost:5432/firmware' \
  $SERVER_IMAGE

echo
echo "Pod and services started. Pod status:"
podman pod ps --filter name="$POD"
echo
echo "Containers in pod:"
podman ps --filter pod="$POD" --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'

echo
echo "To follow logs: podman logs -f ota-backend"
