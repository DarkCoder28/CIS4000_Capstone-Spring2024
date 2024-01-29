#!/bin/bash
rm ./recipe-book
cargo build --target x86_64-unknown-linux-musl --release
cp ./target/x86_64-unknown-linux-musl/release/recipe-book ./recipe-book