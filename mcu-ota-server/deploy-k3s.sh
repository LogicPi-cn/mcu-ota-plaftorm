#!/bin/bash

source ~/.cargo/env

#定义常量
PROJECT_NAME=$(cargo read-manifest | jq -r .name)
UPLOAD_DIR="/apps/${PROJECT_NAME}"
FILE_NAME="${UPLOAD_DIR}/deploy-k3s.yaml"
SSH_USER="root"
SSH_IP="k3s.21up.cn"
 
#首先删除待上传目录的同名文件
ssh ${SSH_USER}@${SSH_IP} "rm -rf ${FILE_NAME}"
 
#确保部署文件目录存在
ssh ${SSH_USER}@${SSH_IP} "mkdir -p ${UPLOAD_DIR}"
 
#远程复制部署文件
scp -r k8s/deploy-k3s.yaml ${SSH_USER}@${SSH_IP}:${FILE_NAME}
 
#远程执行部署命令
# ssh ${SSH_USER}@${SSH_IP} "k3s kubectl apply -f ${FILE_NAME}"
