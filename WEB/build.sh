#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package main
wasm-bindgen --out-dir pkg --web ../target/wasm32-unknown-unknown/release/main.wasm

# Strip binary and optimize it
# wasm-opt -Oz --vacuum --strip-debug

basic-http-server .

