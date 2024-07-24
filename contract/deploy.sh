#!/bin/sh

./build.sh

echo ">> Deploying contract"

near contract deploy succinct-slave.testnet use-file ./target/wasm32-unknown-unknown/release/mystery_box.wasm without-init-call network-config testnet sign-with-keychain send