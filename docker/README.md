# MCU OTA 平台 Docker 使用指南

## 快速开始

### 1. 初始化环境

首次使用时，先初始化环境配置文件：

```bash
./docker-run.sh init
```

这会创建 `.env.docker` 文件，包含所有环境变量配置。

### 2. 启动服务

```bash
./docker-run.sh start
```

等待所有服务启动完成。

### 3. 查看服务状态

```bash
./docker-run.sh status
```

### 4. 查看日志

```bash
# 查看所有日志
./docker-run.sh logs

# 实时跟踪日志
./docker-run.sh logs -f
```

### 5. 停止服务

```bash
./docker-run.sh stop
```

## 命令说明

| 命令 | 说明 |
|------|------|
| `start` | 启动所有服务 |
| `stop` | 停止所有服务 |
| `restart` | 重启所有服务 |
| `status` | 查看服务状态 |
| `logs [-f]` | 查看日志（-f 实时跟踪） |
| `build` | 构建所有镜像 |
| `clean` | 清理容器和镜像 |
| `pull` | 拉取最新镜像 |
| `init` | 初始化环境配置 |

## 服务端口

| 服务 | 端口 | 说明 |
|------|------|------|
| ota-backend | 20000 | Web 后端 API 服务 |
| ota-server | 9999 | TCP 固件服务器 |
| ota-database | 5432 | PostgreSQL 数据库 |

## 环境变量配置

编辑 `.env.docker` 文件可自定义配置：

```bash
# 数据库配置
POSTGRES_USER=craftor
POSTGRES_PASSWORD=3.1415926
POSTGRES_DB=firmware
POSTGRES_PORT=5432

# 后端服务配置
BACKEND_PORT=20000
BACKEND_JWT_SECRET=your-super-secret-jwt-key-change-in-production
BACKEND_JWT_EXPIRED_IN=60
BACKEND_JWT_MAXAGE=60

# Server 服务配置
SERVER_PORT=9999
SERVER_FW_SERVER=http://ota-backend:20000

# 时区配置
TZ=Asia/Shanghai
```

## API 测试

服务启动后，可以使用以下命令测试 API：

```bash
# 健康检查
curl http://localhost:20000/api/healthchecker

# 注册新用户
curl -X POST http://localhost:20000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name":"admin","email":"admin@example.com","password":"password123"}'

# 登录
curl -X POST http://localhost:20000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"password123"}'
```

## 数据持久化

数据库数据存储在 Docker volume `pgdata` 中，即使容器删除数据也不会丢失。

如果需要备份数据：

```bash
docker run --rm \
  -v ota-database_pgdata:/tmp/database \
  -v $(pwd):/backup alpine \
  tar czf /backup/database-backup.tar.gz -C /tmp/database .
```

恢复数据：

```bash
docker run --rm \
  -v ota-database_pgdata:/tmp/database \
  -v $(pwd):/backup alpine \
  tar xzf /backup/database-backup.tar.gz -C /tmp/database
```

## 故障排查

### 服务无法启动

1. 检查 Docker 是否运行：`docker ps`
2. 检查端口是否被占用：`netstat -tlnp | grep 20000`
3. 查看详细日志：`./docker-run.sh logs -f`

### 数据库连接失败

确保数据库健康检查通过：

```bash
docker compose -p docker ps
docker compose -p docker logs ota-database
```

### 重置所有数据

```bash
./docker-run.sh clean
./docker-run.sh start
```
