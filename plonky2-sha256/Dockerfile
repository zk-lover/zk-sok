FROM rust:1.85.0-bookworm

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
  git \
  vim

RUN rustup default nightly-2025-03-15

COPY . /workspace