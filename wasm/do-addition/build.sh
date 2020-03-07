#!/bin/bash

WABT_BIN=$WABT/bin
BINARYEN_BIN=$BINARYEN/bin
TARGET=wasm32-unknown-unknown

NAME=do_addition
BINARY=target/$TARGET/release/$NAME.wasm

cargo build --target $TARGET --release
$WABT_BIN/wasm-strip $BINARY
mkdir -p www
$BINARYEN_BIN/wasm-opt -o www/$NAME.wasm -Oz $BINARY
