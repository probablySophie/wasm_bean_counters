#!/bin/bash

PKG_NAME="wasm_bean_counters" # TODO: figure out how to auto update this to our package name

# Build a non-release version
cargo build --target wasm32-unknown-unknown

# do something???
#wasm-bindgen --target web --no-typescript --out-dir target/wasm32-unknown-unknown/debug target/wasm32-unknown-unknown/debug/wasm_sample.wasm
wasm-bindgen --target web --no-typescript --out-dir target/wasm32-unknown-unknown/debug "target/wasm32-unknown-unknown/debug/${PKG_NAME}.wasm"

# https://github.com/DelphinusLab/zkWasm/issues/145
# ??????????????
# wasm-opt -O3 target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm -o target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm --signext-lowering
wasm-opt -O3 "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm" -o "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm" --signext-lowering


# Shave down everything we don't need
# wasm-gc target/wasm32-unknown-unknown/debug/wasm_sample.wasm
# wasm-gc target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm
wasm-gc "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm"

# Copy the file we made into the www directory
# cp target/wasm32-unknown-unknown/debug/wasm_sample.wasm www/
# cp target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm www/
cp "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm" www/

cp "target/wasm32-unknown-unknown/debug/${PKG_NAME}.js" www/
