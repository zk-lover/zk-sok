# 使用官方的 Go 镜像作为基础镜像
FROM golang:1.22

# 设置工作目录
WORKDIR /workspace

# 将当前目录的所有文件复制到容器中的工作目录
COPY . /workspace

# 设置 Go 模块代理（如果需要）
ENV GOPROXY=https://proxy.golang.org
