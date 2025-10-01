#!/bin/sh
set -e
cargo clean
cargo build
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a53 \
    -nographic \
    -monitor none \
    -serial stdio \
    -kernel ./target/aarch64-unknown-none/debug/lr_os