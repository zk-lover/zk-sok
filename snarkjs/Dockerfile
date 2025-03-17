FROM node:16-bookworm

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    build-essential \
    clang \
    libssl-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.85.0
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install --git https://github.com/iden3/circom.git --tag v2.1.6

RUN npm install -g snarkjs

RUN git clone --branch v2.0.5 https://github.com/iden3/circomlib.git /app/circomlib

COPY . /app

RUN find /app -type f -name "*.sh" -exec chmod +x {} \;

CMD ["bash"]
