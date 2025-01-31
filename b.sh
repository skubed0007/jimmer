#!/bin/bash

# Remove existing bin directory if it exists
if [ -d "bin" ]; then
    rm -rf bin
fi
export PKG_CONFIG_PATH=/usr/lib64/pkgconfig

# Create a new bin directory
mkdir bin

# Build for Linux (64-bit, GNU libc)
echo "Building for Linux (64-bit, GNU libc)..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/jimmer bin/jimmer-linux-gnu-x86_64
