`cargo new --lib do-addition`

Cargo.toml
```toml
[package]
name = "do-addition"
version = "0.1.0"
authors = ["YourName<you@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]
```

build.sh
```bash
#!/bin/bash

WABT_BIN=$WABT/bin
BINARYEN_BIN=$BINARYEN/bin
TARGET=wasm32-unknown-unknown

NAME=do_addition
BINARY=target/$TARGET/release/$NAME.wasm

cargo build --target $TARGET --release
$WABT_BIN/wasm-strip $BINARY
mkdir -p www
$BINARYEN_BIN/wasm -opt -o www/$NAME.wasm -Oz $BINARY
```

We then make this script executable:
`chmod +x build.sh`

We can then build and optimize our Wasm module with `./build.sh`.

Opening most browsers with this HTML page will not work because of browser restrictions on loading Wasm from local files.
The easiest workaround is to serve your Wasm code and use that server to view your example. 
If you have Python installed on your system, you can run a simple server with:

Note the one change from the simplest Python file server is to explicitly set the MIME type for .wasm files. This makes sure browsers handle these files correctly.
We make this script executable as well:
`chmod +x serve.py`

`ls -lh www/do_addition.wasm`

*101 bytes*

`./serve.py`

The Rust compiler knows how to turn your Rust code into the Wasm format. 
The JavaScript engine in the browser knows how to load Wasm code. And, finally, 
there is some execution engine in the browser which an execute that Wasm code when called via the JavaScript interface. 
The one caveat is that the only Rust functions which are valid to expose to Wasm only deal with integers and floating point numbers.


