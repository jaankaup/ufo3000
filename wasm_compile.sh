#!/bin/bash

clear ; cargo run-wasm --example $1 
#clear ; cargo run-wasm --release --example $1 
#clear ; cargo run-wasm --release --example $1 --features webgl 
#scp ./target/wasm-examples/$1/* jaankaup@130.234.208.250:/home/jaankaup/public_html/$1

#RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --features input_debug,texture_debug --no-default-features --target wasm32-unknown-unknown --example "$1"
#wasm-bindgen --out-dir target/generated --web target/wasm32-unknown-unknown/debug/examples/$1.wasm
#wasm-bindgen --no-typescript --out-dir target/generated --web target/wasm32-unknown-unknown/debug/examples/$1.wasm
#RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build -p "$1" --target wasm32-unknown-unknown
#wasm-bindgen --out-dir target/generated --web "target/wasm32-unknown-unknown/debug/$1.wasm"
