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
    cmake

# 安装 Rust 并设置环境变量
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN git clone https://github.com/iden3/circom.git /circom && \
    cd /circom && \
    cargo build --release && \
    cargo install --path circom

# RUN npm config set registry https://registry.npmmirror.com

RUN npm install -g snarkjs

COPY . /app

RUN find /app -type f -name "*.sh" -exec chmod +x {} \;

CMD ["bash"]
