#!/bin/bash

# add cargo path
source ~/.cargo/env

# variables
app_name=$(cargo read-manifest | jq -r .name)
app_version=$(cargo read-manifest | jq -r .version)

docker buildx build \
		--tag logicpi/$app_name:$app_version .\
		--platform linux/amd64 \
		--push
