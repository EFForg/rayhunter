#!/bin/bash -e
pushd daemon/web
    npm run build
popd
cargo build --release --bin rayhunter-daemon
echo "Build complete! Binary available at: target/release/rayhunter-daemon"
