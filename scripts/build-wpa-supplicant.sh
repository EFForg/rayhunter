#!/bin/bash
# Cross-compile wpa_supplicant, wpa_cli, and iw for ARMv7 (musl static).
# Output: tools/build-wpa-supplicant/out/{wpa_supplicant,wpa_cli,iw}
#
# Requires: arm-linux-musleabihf-gcc (brew install FiloSottile/musl-cross/musl-cross)
set -e

WPA_VERSION="2.11"
WPA_URL="https://w1.fi/releases/wpa_supplicant-${WPA_VERSION}.tar.gz"
LIBNL_VERSION="3.11.0"
LIBNL_URL="https://github.com/thom311/libnl/releases/download/libnl${LIBNL_VERSION//\./_}/libnl-${LIBNL_VERSION}.tar.gz"
IW_VERSION="6.9"
IW_URL="https://www.kernel.org/pub/software/network/iw/iw-${IW_VERSION}.tar.xz"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_DIR="$SCRIPT_DIR/../tools/build-wpa-supplicant/out"
BUILD_DIR="/tmp/wpa-supplicant-build-$$"

CC="${CC:-arm-linux-musleabihf-gcc}"
STRIP="${STRIP:-arm-linux-musleabihf-strip}"
HOST="${HOST:-arm-linux-musleabihf}"

if ! command -v "$CC" >/dev/null 2>&1; then
    echo "Error: $CC not found. Install with: brew install FiloSottile/musl-cross/musl-cross"
    exit 1
fi

mkdir -p "$BUILD_DIR" "$OUT_DIR"
SYSROOT="$BUILD_DIR/sysroot"
mkdir -p "$SYSROOT"

echo "Building libnl ${LIBNL_VERSION}..."
curl -Lf "$LIBNL_URL" | tar xz -C "$BUILD_DIR"
cd "$BUILD_DIR/libnl-${LIBNL_VERSION}"
./configure \
    --host="$HOST" \
    CC="$CC" \
    --prefix="$SYSROOT" \
    --enable-static \
    --disable-shared \
    --disable-cli \
    --disable-debug \
    > /dev/null 2>&1
make -j"$(nproc 2>/dev/null || sysctl -n hw.ncpu)" > /dev/null 2>&1
make install > /dev/null 2>&1

echo "Building wpa_supplicant ${WPA_VERSION}..."
cd "$BUILD_DIR"
curl -Lf "$WPA_URL" | tar xz
cd "wpa_supplicant-${WPA_VERSION}/wpa_supplicant"

cat > .config <<'WPACONF'
CONFIG_DRIVER_NL80211=y
CONFIG_LIBNL32=y
CONFIG_CRYPTO=internal
CONFIG_TLS=internal
CONFIG_INTERNAL_LIBTOMMATH=y
CONFIG_INTERNAL_LIBTOMMATH_FAST=y
CONFIG_CTRL_IFACE=y
CONFIG_BACKEND=file
CONFIG_NO_CONFIG_WRITE=y
CONFIG_NO_RANDOM_POOL=y
CONFIG_GETRANDOM=y
WPACONF

NL_CFLAGS="-I${SYSROOT}/include/libnl3"
NL_LIBS="-L${SYSROOT}/lib -lnl-genl-3 -lnl-3 -lpthread -lm"

make CC="$CC" \
    EXTRA_CFLAGS="$NL_CFLAGS" \
    LDFLAGS="-static" \
    LIBS="$NL_LIBS" \
    -j"$(nproc 2>/dev/null || sysctl -n hw.ncpu)"

echo "Stripping..."
$STRIP wpa_supplicant wpa_cli
cp wpa_supplicant wpa_cli "$OUT_DIR/"

echo "Building iw ${IW_VERSION}..."
cd "$BUILD_DIR"
curl -Lf "$IW_URL" | tar xJ
cd "iw-${IW_VERSION}"
make CC="$CC" \
    PKG_CONFIG_PATH="$SYSROOT/lib/pkgconfig" \
    LDFLAGS="-static" \
    -j"$(nproc 2>/dev/null || sysctl -n hw.ncpu)"
$STRIP iw
cp iw "$OUT_DIR/"

rm -rf "$BUILD_DIR"

echo "Done. Binaries in $OUT_DIR:"
ls -lh "$OUT_DIR"/{wpa_supplicant,wpa_cli,iw}
