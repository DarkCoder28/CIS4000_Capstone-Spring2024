#!/bin/bash
# SCRIPT_DIR="$(dirname "$(realpath "$0")")"
# CLIENT_DIR="$SCRIPT_DIR/../client/"
# cd $CLIENT_DIR

# # Windows Toolchain: x86_64-pc-windows-msvc
# # Linux Toolchain: x86_64-unknown-linux-gnu
# cargo build --release --target x86_64-unknown-linux-gnu
# cargo build --release --target x86_64-pc-windows-gnu

# cp -f $CLIENT_DIR/target/x86_64-unknown-linux-gnu/release/gwynedd_valley_client $SCRIPT_DIR/src/static/clients/gwynedd_valley_client-linux
# cp -f $CLIENT_DIR/target/x86_64-pc-windows-gnu/release/gwynedd_valley_client.exe $SCRIPT_DIR/src/static/clients/gwynedd_valley_client-windows.exe

# cd $SCRIPT_DIR
MONGODB_URI="mongodb://192.168.1.64:27017" cargo run