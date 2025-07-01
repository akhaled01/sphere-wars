#!/bin/bash

export RUST_LOG=info

get_font() {
    if [ ! -f "./client/assets/SpaceMono-Regular.ttf" ]; then
        mkdir -p ./client/assets
        curl -s -o ./client/assets/SpaceMono-Regular.ttf https://github.com/google/fonts/blob/main/ofl/spacemono/SpaceMono-Regular.ttf
    fi
}

get_font

cargo build --bin server --release
cargo build --bin client --release

# extract the binaries and place in bin folder
mkdir -p bin
cp target/release/server bin/server
cp target/release/client bin/client

echo "Build complete"