#!/bin/sh
cargo build --target wasm32-unknown-unknown --release

mkdir -p res

cp target/wasm32-unknown-unknown/release/mystery_box.wasm res/mystery_box.wasm
