# MCU OTA 前端实现完成

## 已创建的文件

### 项目配置文件
- `package.json` - 项目依赖配置
- `vite.config.ts` - Vite 构建配置
- `tsconfig.json` - TypeScript 配置
- `tsconfig.node.json` - TypeScript Node 配置
- `index.html` - HTML 入口

### 源代码文件
- `src/main.ts` - 应用入口
- `src/App.vue` - 根组件
- `src/vite-env.d.ts` - TypeScript 环境声明

### 路由配置
- `src/router/index.ts` - Vue Router 配置

### API 客户端
- `src/api/index.ts` - Axios 实例配置
- `src/api/types.ts` - TypeScript 类型定义
- `src/api/auth.ts` - 认证 API
- `src/api/firmware.ts` - 固件 API
- `src/api/history.ts` - 升级历史 API

### 状态管理
- `src/stores/user.ts` - 用户状态管理

### 页面组件
- `src/views/Login.vue` - 登录页面
- `src/views/Dashboard.vue` - 仪表盘页面
- `src/views/Firmware.vue` - 固件管理页面
- `src/views/History.vue` - 升级历史页面

### 公共组件
- `src/components/Layout.vue` - 布局组件

### Docker 配置
- `Dockerfile` - Docker 镜像构建
- `nginx.conf` - Nginx 配置
- `docker-entrypoint.sh` - 启动脚本
- `.dockerignore` - Docker 忽略文件
- `.gitignore` - Git 忽略文件
- `.npmignore` - NPM 忽略文件

### 文档
- `README.md` - 项目说明文档

### 环境配置
- `.env` - 环境变量
- `.env.example` - 环境变量示例

## Docker 集成

已更新以下文件：

1. `docker/docker-compose.yml` - 添加 `ota-frontend` 服务
2. `docker-run.sh` - 添加前端相关命令

## 使用方法

### 开发模式

```bash
cd ota-frontend
npm install
npm run dev
```

访问 http://localhost:5173

### Docker 部署

```bash
# 构建前端镜像
./docker-run.sh build-frontend

# 启动前端服务
./docker-run.sh start-frontend

# 或使用 docker-compose
cd docker
docker compose up -d ota-frontend
```

访问 http://localhost:8080

### 完整部署

```bash
# 启动所有服务
./docker-run.sh start
```

## 功能特性

1. **登录认证**
   - JWT Token 认证
   - Cookie 存储
   - 自动登录

2. **仪表盘**
   - 固件总数统计
   - 设备总数统计
   - 升级成功/失败统计
   - 最近升级记录表格

3. **固件管理**
   - 固件列表展示
   - 上传固件（支持拖拽）
   - 查看详情
   - 删除固件
   - 搜索功能

4. **升级历史**
   - 历史记录表格
   - 按设备 ID 筛选
   - 按固件代号筛选
   - 按状态筛选
   - 按时间范围筛选

## 技术栈

- Vue 3.4.x
- Vite 5.x
- Pinia 2.1.x
- Vue Router 4.3.x
- Axios 1.6.x
- Arco Design 2.55.x
- Day.js 1.11.x
- js-cookie 3.0.x

## 默认端口

- 开发模式：5173
- Docker 部署：8080
- 后端 API：20000
- 数据库：5432
- TCP 服务器：9999

## 注意事项

1. 首次使用需要注册账号
2. 确保后端服务已启动
3. Docker 部署时确保网络通畅
