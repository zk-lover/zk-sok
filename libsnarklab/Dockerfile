FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

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
    && apt-get clean

WORKDIR /app

COPY . /app
