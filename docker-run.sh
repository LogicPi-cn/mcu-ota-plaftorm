#!/bin/bash
#===============================================================================
# 文件名：docker-run.sh
# 描述：MCU OTA 平台 Docker 一键启动/停止/管理脚本
# 用法：
#   ./docker-run.sh start    - 启动所有服务
#   ./docker-run.sh stop     - 停止所有服务
#   ./docker-run.sh restart  - 重启所有服务
#   ./docker-run.sh status   - 查看服务状态
#   ./docker-run.sh logs     - 查看所有日志
#   ./docker-run.sh build    - 构建所有镜像
#   ./docker-run.sh clean    - 清理容器和镜像
#===============================================================================

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 脚本目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="${SCRIPT_DIR}/docker/docker-compose.yml"
ENV_FILE="${SCRIPT_DIR}/.env.docker"

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 显示使用帮助
show_help() {
    cat << EOF
MCU OTA 平台 Docker 管理脚本

用法：$0 <命令> [选项]

命令:
    start       启动所有服务
    stop        停止所有服务
    restart     重启所有服务
    status      查看服务状态
    logs        查看所有日志 (支持 -f 参数跟踪)
    build       构建所有镜像
    clean       清理容器和镜像
    pull        拉取最新镜像
    init        初始化环境（创建 .env.docker 文件）

选项:
    -h, --help      显示帮助信息
    -f, --follow    跟踪日志输出 (仅 logs 命令)

示例:
    $0 start            # 启动所有服务
    $0 logs -f          # 跟踪查看日志
    $0 stop             # 停止所有服务

EOF
}

# 检查 Docker 是否安装
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker 未安装，请先安装 Docker"
        exit 1
    fi

    if ! command -v docker compose &> /dev/null && ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose 未安装，请先安装 Docker Compose"
        exit 1
    fi

    if ! docker info &> /dev/null; then
        print_error "Docker 服务未运行，请启动 Docker 服务"
        exit 1
    fi
}

# COMPOSE 命令别名
get_compose_cmd() {
    if command -v docker compose &> /dev/null; then
        echo "docker compose"
    else
        echo "docker-compose"
    fi
}

# 初始化环境文件
init_env() {
    if [ -f "${ENV_FILE}" ]; then
        print_warning ".env.docker 文件已存在"
        read -p "是否覆盖？(y/N): " confirm
        if [ "$confirm" != "y" ]; then
            print_info "取消初始化"
            return 0
        fi
    fi

    cat > "${ENV_FILE}" << 'ENVEOF'
# MCU OTA 平台 Docker 环境变量配置
# 复制此文件为 .env.docker 并根据需要修改

# ========== 数据库配置 ==========
POSTGRES_USER=craftor
POSTGRES_PASSWORD=3.1415926
POSTGRES_DB=firmware
POSTGRES_PORT=5432

# ========== 后端服务配置 ==========
BACKEND_PORT=20000
BACKEND_JWT_SECRET=your-super-secret-jwt-key-change-in-production
BACKEND_JWT_EXPIRED_IN=60
BACKEND_JWT_MAXAGE=60

# ========== Server 服务配置 ==========
SERVER_PORT=9999
SERVER_FW_SERVER=http://ota-backend:20000

# ========== 时区配置 ==========
TZ=Asia/Shanghai
ENVEOF

    print_success "环境文件已创建：${ENV_FILE}"
    print_info "请根据需要修改环境变量配置"
}

# 启动服务
start_services() {
    print_info "启动 MCU OTA 平台服务..."

    # 检查环境文件
    if [ ! -f "${ENV_FILE}" ]; then
        print_warning "环境文件不存在，创建默认配置..."
        init_env
    fi

    # 加载环境变量
    set -a
    source "${ENV_FILE}"
    set +a

    # 启动服务
    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) up -d

    print_success "服务启动完成!"
    print_info "使用 '$0 status' 查看服务状态"
    print_info "使用 '$0 logs -f' 查看日志"
}

# 停止服务
stop_services() {
    print_info "停止 MCU OTA 平台服务..."

    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) down

    print_success "服务已停止"
}

# 重启服务
restart_services() {
    stop_services
    sleep 2
    start_services
}

# 查看服务状态
show_status() {
    print_info "MCU OTA 平台服务状态:"
    echo ""

    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) ps
}

# 查看日志
show_logs() {
    local follow=""
    if [ "$1" == "-f" ] || [ "$1" == "--follow" ]; then
        follow="-f"
    fi

    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) logs ${follow}
}

# 构建镜像
build_images() {
    print_info "构建所有 Docker 镜像..."

    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) build --no-cache

    print_success "镜像构建完成"
}

# 拉取镜像
pull_images() {
    print_info "拉取最新 Docker 镜像..."

    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) pull

    print_success "镜像拉取完成"
}

# 清理容器和镜像
clean_all() {
    print_warning "此操作将删除所有容器和镜像!"
    read -p "确定要继续吗？(y/N): " confirm

    if [ "$confirm" != "y" ]; then
        print_info "取消操作"
        return 0
    fi

    cd "${SCRIPT_DIR}/docker"
    $(get_compose_cmd) down -v --rmi all --remove-orphans

    print_success "清理完成"
}

# 主函数
main() {
    # 如果没有参数，显示帮助
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    # 检查 Docker
    check_docker

    case "$1" in
        start)
            start_services
            ;;
        stop)
            stop_services
            ;;
        restart)
            restart_services
            ;;
        status)
            show_status
            ;;
        logs)
            shift
            show_logs "$@"
            ;;
        build)
            build_images
            ;;
        clean)
            clean_all
            ;;
        pull)
            pull_images
            ;;
        init)
            init_env
            ;;
        -h|--help)
            show_help
            ;;
        *)
            print_error "未知命令：$1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
