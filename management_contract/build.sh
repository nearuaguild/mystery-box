#!/bin/sh
cargo build --target wasm32-unknown-unknown --release

mkdir -p res

cp target/wasm32-unknown-unknown/release/mystery_box_management_contract.wasm res/management_contract.wasm
