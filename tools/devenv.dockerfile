FROM rust:1.86-bullseye

RUN rustup target add armv7-unknown-linux-musleabihf
