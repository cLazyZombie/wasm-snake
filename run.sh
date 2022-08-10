#!/bin/sh

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_rust_game_of_life.wasm --out-dir ./out/ --target web
http-server . -p 8081 -c-1
