FROM rust:slim-bullseye

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
  git \
  vim

RUN rustup default nightly 

COPY . /workspace