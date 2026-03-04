# 使用 Rust 官方 Docker 镜像作为基础镜像
FROM rust:latest AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

# 国内源
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources

# 替换默认的 Debian 软件源
RUN apt-get update && \
    apt-get install -y cmake build-essential

# 在容器中创建一个工作目录
WORKDIR /app/ota-backend

# 复制根目录的 Cargo 文件
COPY ../Cargo.toml ../Cargo.lock ./

# 复制后端代码
COPY . .

# 使用 Cargo 构建项目
RUN cargo build --release

# 从新的镜像开始构建
FROM ubuntu:24.04

ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

# 安装必要的依赖
RUN apt-get update && \
    apt-get install -y libpq5 dialog apt-utils && \
    rm -rf /var/lib/apt/lists/*

# 创建一个新的工作目录
WORKDIR /app
RUN mkdir -p /app/ftp

# 环境变量 - 通过 docker-compose 覆盖
ENV FW_DB=postgres://craftor:3.1415926@ota-database:5432/firmware
ENV DATABASE_URL=postgres://craftor:3.1415926@ota-database:5432/firmware

# 复制编译后的应用程序到新的工作目录
COPY --from=builder /app/ota-backend/target/release/ota-backend .

EXPOSE 20000/tcp

# 在容器中运行项目
CMD ["./ota-backend", "--port", "20000"]
