# MCU OTA 前端

MCU OTA 平台的前端界面，基于 Vue 3 + Vite + Arco Design 构建。

## 技术栈

- **Vue 3** - 渐进式 JavaScript 框架
- **Vite** - 下一代前端构建工具
- **Pinia** - Vue 3 专属状态管理库
- **Vue Router** - 官方路由管理器
- **Arco Design** - 字节跳动开源的企业级 UI 组件库
- **Axios** - HTTP 客户端
- **Day.js** - 日期处理库

## 功能特性

- **登录认证** - JWT Token 认证，支持自动登录
- **仪表盘** - 显示固件统计、设备统计、升级统计
- **固件管理** - 上传、查看、删除固件
- **升级历史** - 查看升级历史记录，支持多维度筛选

## 开发环境运行

### 1. 安装依赖

```bash
npm install
```

### 2. 配置环境变量（可选）

在项目根目录创建 `.env` 文件：

```env
VITE_API_BASE_URL=/api
```

### 3. 启动开发服务器

```bash
npm run dev
```

访问 http://localhost:5173

### 4. 配置 API 代理

开发环境下，Vite 已配置 `/api` 代理到 `http://localhost:20000`。

请确保后端服务已启动。

## Docker 部署

### 构建镜像

```bash
./docker-run.sh build-frontend
```

### 启动服务

```bash
./docker-run.sh start-frontend
```

访问 http://localhost:8080

### 完整部署

```bash
./docker-run.sh start
```

启动所有服务（数据库、后端、前端、TCP 服务器）

## 项目结构

```
ota-frontend/
├── src/
│   ├── api/              # API 接口
│   │   ├── index.ts      # Axios 实例配置
│   │   ├── auth.ts       # 认证 API
│   │   ├── firmware.ts   # 固件 API
│   │   └── history.ts    # 历史 API
│   ├── components/       # 公共组件
│   │   └── Layout.vue    # 布局组件
│   ├── views/            # 页面视图
│   │   ├── Login.vue     # 登录页
│   │   ├── Dashboard.vue # 仪表盘
│   │   ├── Firmware.vue  # 固件管理
│   │   └── History.vue   # 升级历史
│   ├── router/           # 路由配置
│   │   └── index.ts
│   ├── stores/           # Pinia 状态管理
│   │   └── user.ts       # 用户状态
│   ├── App.vue           # 根组件
│   └── main.ts           # 应用入口
├── public/
├── index.html
├── package.json
├── vite.config.ts
├── tsconfig.json
├── Dockerfile
├── nginx.conf
└── docker-entrypoint.sh
```

## API 接口

### 认证 API
- `POST /api/auth/login` - 用户登录
- `GET /api/auth/logout` - 用户登出
- `GET /api/users/me` - 获取当前用户信息

### 固件 API
- `GET /api/firmware` - 获取固件列表
- `POST /api/firmware` - 上传固件
- `GET /api/firmware/:id` - 获取固件详情
- `PATCH /api/firmware/:id` - 更新固件
- `DELETE /api/firmware/:id` - 删除固件

### 历史 API
- `GET /api/history` - 获取升级历史列表
- `POST /api/history` - 创建升级历史
- `GET /api/history/:id` - 获取历史记录详情
- `PATCH /api/history/:id` - 更新历史记录
- `DELETE /api/history/:id` - 删除历史记录

## 默认账号

首次使用需要通过 API 注册账号：

```bash
curl -X POST http://localhost:20000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name":"admin","email":"admin@example.com","password":"123456"}'
```

## 许可证

MIT
