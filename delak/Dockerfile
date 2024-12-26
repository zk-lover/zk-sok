# 使用官方的 Go 镜像作为基础镜像
FROM rust:slim-bullseye

# 设置工作目录
WORKDIR /workspace

RUN apt-get update && apt-get install -y \
  git \
  vim

# 将当前目录的所有文件复制到容器中的工作目录
COPY . /workspace