FROM rust:1.85.0-bookworm

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
  git \
  vim

RUN rustup default nightly 

COPY . /workspace