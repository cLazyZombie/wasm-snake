
# build
cargo build --target wasm-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_snake.wasm --out-dir .

# run
http-server -p 8080 -c-1 .