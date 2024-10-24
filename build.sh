#!/bin/bash

PKG_NAME="wasm_bean_counters" # TODO: figure out how to auto update this to our package name
# TODO: Come through and have proper error messages

# Build a non-release version
if cargo build --target wasm32-unknown-unknown ; then 
	echo "";
else
	printf "${red}Failed to build for target: wasm32-unknown-unknown${normal}\n"
	return 1
fi

# do something???
#wasm-bindgen --target web --no-typescript --out-dir target/wasm32-unknown-unknown/debug target/wasm32-unknown-unknown/debug/wasm_sample.wasm
if wasm-bindgen --target web --no-typescript --out-dir target/wasm32-unknown-unknown/debug "target/wasm32-unknown-unknown/debug/${PKG_NAME}.wasm"; then
	printf "";
else
	return 1
fi

# https://github.com/DelphinusLab/zkWasm/issues/145
# ??????????????
# wasm-opt -O3 target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm -o target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm --signext-lowering
if wasm-opt -O3 "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm" -o "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm" --signext-lowering ; then
	printf "";
else
	printf "${red}????${normal}\n"
	return 1
fi


# Shave down everything we don't need
if wasm-gc "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm"; then
	printf ""
else
	printf "${red}Failed to run wasm-gc${normal}\n"
	return 1
fi

# Copy the file we made into the www directory
# cp target/wasm32-unknown-unknown/debug/wasm_sample.wasm www/
# cp target/wasm32-unknown-unknown/debug/wasm_sample_bg.wasm www/
if cp "target/wasm32-unknown-unknown/debug/${PKG_NAME}_bg.wasm" www/ ; then
	printf ""
else
	printf "${red}Failed to copy main WASM file${normal}\n"
	return 1
fi

if cp "target/wasm32-unknown-unknown/debug/${PKG_NAME}.js" www/ ; then
	printf ""
else
	printf "${red}Failed to copy WASM JS file${normal}\n"
	return 1
fi
