#!/bin/sh
set -e

# 环境变量默认值设置
BACKEND_URL="${BACKEND_URL:-http://ota-backend:20000}"

echo "Starting MCU OTA Frontend..."
echo "Backend URL: $BACKEND_URL"

# 启动 Nginx
exec nginx -g "daemon off;"
