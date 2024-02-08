#!/bin/bash
SCRIPT_DIR="$(dirname "$(realpath "$0")")"
CLIENT_DIR="$SCRIPT_DIR/../client/"
cd $CLIENT_DIR

# Windows Toolchain: x86_64-pc-windows-msvc
# Linux Toolchain: x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

cp -f $CLIENT_DIR/target/x86_64-unknown-linux-gnu/release/cis4000_capstone-spring2024_client $SCRIPT_DIR/src/static/clients/client-linux
cp -f $CLIENT_DIR/target/x86_64-pc-windows-gnu/release/cis4000_capstone-spring2024_client.exe $SCRIPT_DIR/src/static/clients/client-windows.exe

cd $SCRIPT_DIR
MONGODB_URI="mongodb://192.168.1.64:27017" PK_ID="home.thesheerans.com" PK_ORIGIN="https://home.thesheerans.com:3333" cargo run