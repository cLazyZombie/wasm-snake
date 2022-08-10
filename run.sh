#!/bin/sh

set -e
set -x

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_snake.wasm  --target web --out-dir ./out
http-server . -p 8081 -c-1
