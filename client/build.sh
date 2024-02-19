#!/bin/bash

rm ./out/*

# Windows Toolchain: x86_64-pc-windows-msvc
# Linux Toolchain: x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

mkdir -p ./out/
cp -f ./target/x86_64-unknown-linux-gnu/release/gwynedd_valley_client ./out/gwynedd_valley_client-linux
cp -f ./target/x86_64-pc-windows-gnu/release/gwynedd_valley_client.exe ./out/gwynedd_valley_client-windows.exe

# rm ./gwynedd-valley
# cargo build --release --target x86_64-unknown-linux-musl
# cargo build --release --target x86_64-pc-windows-gnu
# mkdir ./out
# cp ./target/x86_64-unknown-linux-musl/release/gwynedd_valley_server ./out/gwynedd-valley
# cp ./target/x86_64-unknown-linux-musl/release/gwynedd_valley_server ./out/gwynedd-valley