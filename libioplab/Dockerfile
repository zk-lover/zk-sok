# 使用 Ubuntu 20.04 作为基础镜像
FROM ubuntu:20.04

# 避免交互式提示
ENV DEBIAN_FRONTEND=noninteractive

# 设置编译器标志
ENV CXXFLAGS="-Wno-error=deprecated-declarations -Wno-error=unused-result"

# 安装必要的依赖
RUN apt-get update && apt-get install -y \
    git \
    build-essential \
    cmake \
    libgmp3-dev \
    python3 \
    python3-markdown \
    libboost-program-options-dev \
    libsodium-dev \ 
    libsodium23 \   
    libssl-dev \
    pkg-config \
    libprocps-dev \
    libgtest-dev \   
    && cd /usr/src/gtest \
    && cmake CMakeLists.txt \
    && make \
    && cp lib/*.a /usr/lib \
    && apt-get clean

# 设置工作目录
WORKDIR /app

# 复制项目文件（包括已经初始化的子模块）
COPY . /app
