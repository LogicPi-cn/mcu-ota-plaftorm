#!/bin/bash

# 登录 Docker
docker login

# 定义白名单数组
whitelist=("ota-database" "ota-server" "ota-backend")

# 遍历工作区中的每个应用
for dir in $(find . -maxdepth 1 -type d ! -name ".*" ! -name "target" -print); do
    # 检查目录下是否存在 Cargo.toml 文件
    if [ -f "$dir/Cargo.toml" ]; then
        # 进入应用的目录
        cd $dir

        # 使用 cargo 命令和 jq 工具获取应用的名称
        app_name=$(cargo read-manifest | jq -r .name)
        # 使用 cargo 命令和 jq 工具获取应用的版本号
        app_version=$(cargo read-manifest | jq -r .version)
        
        # 返回上一层目录
        cd ..

        # 检查应用的名称是否在白名单中
        if printf '%s\n' "${whitelist[@]}" | grep -q -P "^$app_name$"; then
            # 打包并上传
            docker buildx build \
                    -f Dockerfile.$app_name \
                    --tag logicpi/$app_name \
                    --tag logicpi/$app_name:$app_version .\
                    --platform linux/amd64 \
                    --push
        fi
    fi
done
