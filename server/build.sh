#!/bin/bash
SCRIPT_DIR="$(dirname "$(realpath "$0")")"
cd $SCRIPT_DIR/../client/
cargo build --release
cp ./target/wasm32-unknown-unknown/release/cis4000_capstone-spring2024_client.wasm $SCRIPT_DIR/src/static/scripts/client.wasm
cd $SCRIPT_DIR
rm ./cis4000-project
cargo build --target x86_64-unknown-linux-musl --release
cp ./target/x86_64-unknown-linux-musl/release/cis4000_capstone-spring2024_server ./cis4000-project