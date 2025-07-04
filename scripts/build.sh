#!/bin/bash

export RUST_LOG=info

rm -rf ./bin


get_font() {
    if [ ! -f "./bin/assets/SpaceMono-Regular.ttf" ]; then
        mkdir -p ./bin/assets
        curl -L -s -o ./bin/assets/SpaceMono-Regular.ttf https://raw.githubusercontent.com/google/fonts/refs/heads/main/ofl/spacemono/SpaceMono-Regular.ttf
    fi
}

get_font

cargo fmt

# Build both binaries in parallel
cargo build --bin server --release &
server_pid=$!
cargo build --bin client --release &
client_pid=$!

# Wait for both builds to complete
wait $server_pid
wait $client_pid

mkdir -p bin
cp target/release/server bin/server
cp target/release/client bin/client

# Create aliases file
cat > .aliases << 'EOF'
alias war-server="./bin/server"
alias war-client="./bin/client"
EOF

echo "Build complete!"
echo "To use aliases, run: source .aliases"
echo "Then you can use 'war-server' and 'war-client' commands"
