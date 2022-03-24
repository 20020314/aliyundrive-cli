FROM rust:1.58-alpine3.14 AS builder

WORKDIR /home/root/src
USER root

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
RUN apk add git musl-dev linux-headers libressl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN git clone https://github.com/zf1976/pancli.git
RUN cd ./pancli/libs/qrcode_term && cargo run --example encode_string --target x86_64-unknown-linux-musl