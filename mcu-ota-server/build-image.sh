#!/bin/bash

# add cargo path
source ~/.cargo/env

# variables
app_name=$(cargo read-manifest | jq -r .name)
app_version=$(cargo read-manifest | jq -r .version)

docker login -u craftor -p Operati0n reg.21up.cn

docker buildx build \
		--tag reg.21up.cn/iot/$app_name:$app_version .\
		--platform linux/amd64 \
		--push
