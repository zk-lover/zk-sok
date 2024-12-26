# 使用官方 Node.js 镜像作为基础镜像
FROM node:20

# 设置工作目录
WORKDIR /workspace

# 安装 snarkjs
RUN npm install -g snarkjs

# 复制项目代码到容器
COPY . /workspace
