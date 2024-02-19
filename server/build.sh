#!/bin/bash

rm ./gwynedd-valley
cargo build --target x86_64-unknown-linux-musl --release
cp ./target/x86_64-unknown-linux-musl/release/gwynedd_valley_server ./gwynedd-valley