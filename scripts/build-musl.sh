#!/bin/sh
# Static release builds via zig. Usage: scripts/build-musl.sh x86_64|aarch64
set -e
A=${1:-x86_64}
T=$A-unknown-linux-musl
U=$(echo $T | tr 'a-z-' 'A-Z_')
export PATH=~/bin:~/.cargo/bin:$PATH
export CC_$(echo $T | tr '-' '_')=zcc-$A-musl AR_$(echo $T | tr '-' '_')=zar
export CARGO_TARGET_${U}_LINKER=zcc-$A-musl
export CARGO_TARGET_${U}_RUSTFLAGS="-C link-self-contained=no"
cargo build --release --target $T
mkdir -p dist && cp ${CARGO_TARGET_DIR:-target}/$T/release/rimepanel dist/rimepanel-linux-$A
