#!/bin/bash
# Bootstrap the Dotlin compiler

set -e

echo "Building Dotlin compiler..."
cargo build --release --bin dotlinc

echo "Installing compiler..."
cargo