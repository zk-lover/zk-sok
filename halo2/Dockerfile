FROM rust:1.85.0-slim-bullseye

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
  git \
  vim

COPY . /workspace