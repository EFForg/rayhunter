FROM ubuntu:20.04 AS builder
ENV DEBIAN_FRONTEND=noninteractive

RUN dpkg --add-architecture armhf && \
    echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu focal main universe" > /etc/apt/sources.list && \
    echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu focal-updates main universe" >> /etc/apt/sources.list && \
    echo "deb [arch=amd64] http://security.ubuntu.com/ubuntu focal-security main universe" >> /etc/apt/sources.list && \
    echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal main universe" >> /etc/apt/sources.list && \
    echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal-updates main universe" >> /etc/apt/sources.list && \
    echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal-security main universe" >> /etc/apt/sources.list && \
    apt-get update && apt-get install -y \
    build-essential gcc-arm-linux-gnueabihf pkg-config wget \
    libnl-3-dev:armhf libnl-genl-3-dev:armhf \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
RUN wget https://w1.fi/releases/wpa_supplicant-2.11.tar.gz && \
    tar -xf wpa_supplicant-2.11.tar.gz

WORKDIR /build/wpa_supplicant-2.11/wpa_supplicant

RUN printf '%s\n' \
    "CONFIG_DRIVER_NL80211=y" \
    "CONFIG_LIBNL32=y" \
    "CONFIG_CRYPTO=internal" \
    "CONFIG_TLS=internal" \
    "CONFIG_INTERNAL_LIBTOMMATH=y" \
    "CONFIG_INTERNAL_LIBTOMMATH_FAST=y" \
    "CONFIG_CTRL_IFACE=y" \
    "CONFIG_BACKEND=file" \
    "CONFIG_NO_CONFIG_WRITE=y" \
    "CONFIG_NO_RANDOM_POOL=y" \
    "CONFIG_GETRANDOM=y" \
    > .config

RUN make CC=arm-linux-gnueabihf-gcc \
    EXTRA_CFLAGS="$(arm-linux-gnueabihf-pkg-config --cflags libnl-3.0 libnl-genl-3.0)" \
    LDFLAGS="-static" \
    LIBS="$(arm-linux-gnueabihf-pkg-config --libs --static libnl-3.0 libnl-genl-3.0) -lm -lpthread" \
    -j$(nproc)

FROM scratch AS export
COPY --from=builder /build/wpa_supplicant-2.11/wpa_supplicant/wpa_supplicant /
COPY --from=builder /build/wpa_supplicant-2.11/wpa_supplicant/wpa_cli /
