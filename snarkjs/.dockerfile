# 使用 Node.js 官方镜像作为基础镜像
FROM node:16-buster

# 设置工作目录
WORKDIR /app

# 安装 Rust 和 Cargo（用于构建 Circom）
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    clang \
    libssl-dev \
    cmake \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 安装 snarkjs
RUN npm install -g snarkjs

# 克隆并编译 Circom
RUN git clone https://github.com/iden3/circom.git /circom && \
    cd /circom && \
    cargo build --release && \
    cargo install --path .

# 设置 PATH 环境变量
ENV PATH="/root/.cargo/bin:/app:/circom/target/release:${PATH}"

# 将当前工作目录复制到容器中的工作目录
COPY . /app

# 设置默认命令
CMD ["bash"]
