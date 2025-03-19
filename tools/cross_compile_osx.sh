#!/bin/bash -e

# This script expects to be run from the root of the rayhunter repo.

# Dependencies
if [ `whoami` != "root" ]; then
	# only use sudo if we are not root
	sudo=sudo
fi
$sudo apt update && $sudo apt install -y curl clang cmake git patch python3 libssl-dev tar lzma-dev libxml2-dev xz-utils bzip2 cpio zlib1g-dev xz-utils bzip2 cpio zlib1g-dev lzma-dev libbz2-1.0 file

# Compile osxcross
git clone https://github.com/tpoechtrager/osxcross
cd osxcross/
# MacOS SDK
curl -L https://github.com/roblabla/MacOSX-SDKs/releases/download/13.3/MacOSX13.3.sdk.tar.xz > tarballs/MacOSX13.3.sdk.tar.xz
UNATTENDED=1 ./build.sh
export PATH=$(pwd)/target/bin:$PATH
# Verifying our x86_64 compiler works as expected
o64-clang++ oclang/test.cpp -O3 -o test
file test
# Verifying out ARM compiler works as expeected
oa64-clang++ oclang/test.cpp -O3 -o test
file test
cd ..

# Now we compile serial for rayhunter
cd serial/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh ; sh rustup.sh -y
. "$HOME/.cargo/env"
rustup default stable ; rustup target add aarch64-apple-darwin x86_64-apple-darwin
export SDKROOT=../../osxcross/target/SDK/MacOSX13.3.sdk
export PATH=$PATH:~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/
export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=rust-lld
export CC=o64-clang CXX=o64-clang++
cargo build --target x86_64-apple-darwin
file ../target/x86_64-apple-darwin/debug/serial
cargo build --target x86_64-apple-darwin --release
file ../target/x86_64-apple-darwin/release/serial

export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=../osxcross/target/bin/x86_64-apple-darwin22.4-cc
export CC=oa64-clang CXX=oa64-clang++
cargo build --target=aarch64-apple-darwin
file ../target/aarch64-apple-darwin/debug/serial
cargo build --target=aarch64-apple-darwin --release
file ../target/aarch64-apple-darwin/release/serial
