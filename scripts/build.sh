#!/bin/bash

export RUST_LOG=info

cargo build --bin server --release
cargo build --bin client --release

# extract the binaries and place in bin folder
mkdir -p bin
cp target/release/server bin/server
cp target/release/client bin/client

echo "Build complete"