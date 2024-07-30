#!/bin/sh
cargo build --target wasm32-unknown-unknown --release

mkdir -p res

cp target/wasm32-unknown-unknown/release/mystery_box.wasm src/tests/integration_tests/wasms/mystery_box.wasm
