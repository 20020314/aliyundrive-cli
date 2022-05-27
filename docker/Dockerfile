FROM rust:1.61-alpine AS builder

WORKDIR /home/root/src
USER root

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
RUN apk add git musl-dev linux-headers libressl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN git clone https://github.com/gngpp/aliyundrive-cli.git
RUN cd ./aliyundrive-cli && cargo run encode_string --target x86_64-unknown-linux-musl