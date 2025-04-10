FROM rust:1.86-bullseye

RUN apt-get update
RUN apt-get install -y build-essential libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf
