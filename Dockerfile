FROM debian:11
FROM rust:latest AS builder
RUN rustup install stable-x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /app

RUN git clone https://github.com/wrp801/datagen.git

WORKDIR /app/datagen

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
# ENV CC_x86_64_unknown_linux_musl=gcc-x86-64-linux-gnu
# ENV CC_x86_64-unknown-linux-musl=gcc-x86-64-linux-gnu
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc
RUN cargo build --target x86_64-unknown-linux-musl --release


