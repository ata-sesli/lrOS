#!/bin/sh
set -e
cargo clean
cargo build --release

echo "Starting QEMU..."
qemu-system-aarch64 \
    -machine virt \
    -cpu max \
    -smp 1 \
    -m 512M \
    -nographic \
    -monitor none \
    -serial stdio \
    -kernel ./target/aarch64-unknown-none/release/lr_os