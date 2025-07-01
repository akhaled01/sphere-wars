#!/bin/bash

export RUST_LOG=info

rm ./bin


get_font() {
    if [ ! -f "./bin/assets/SpaceMono-Regular.ttf" ]; then
        mkdir -p ./bin/assets
        curl -L -s -o ./bin/assets/SpaceMono-Regular.ttf https://raw.githubusercontent.com/google/fonts/refs/heads/main/ofl/spacemono/SpaceMono-Regular.ttf
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
