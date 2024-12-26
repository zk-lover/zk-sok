# 使用官方的 Go 镜像作为基础镜像
FROM gcc:12

# 安装必要的依赖项
RUN apt-get update && apt-get install -y \
    cmake \
    git \
    libgmp-dev \
    libprocps-dev \
    python3 \
    python3-pip \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /workspace

# 将当前目录的所有文件复制到容器中的工作目录
COPY . /workspace

# 设置 Go 模块代理（如果需要）
ENV GOPROXY=https://proxy.golang.org
