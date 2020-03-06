Init
`cargo new --lib do-nothing`

We need to specify that our library will be a *C dynamic library* as this is the type of compilation format that Wasm uses.
Depending on the target architecture this is also how you would build a `.so` on Linux or `.dylib` on Mac OS. 
We do this by adding the lib section to our Cargo.toml:
`
[package]
name = "do-nothing"
version = "0.1.0"
authors = ["Conal RYAN <conaleryan@gmail.com>"]
edition = "2018"

lib]
crate-type = ["cdylib"]

[dependencies]
`

We can then remove all of the code in `src/lib.rs` to leave a blank file.
  cr. 
  Why? There is only an auto generated test. 
  Are the test included in a release build? 
  Shouldn't that be removed for a release build to reduce the final artifact size? 

`cargo build --target wasm32-unknown-unknown --release`

By default, Cargo puts the build artifacts in `target/<target-triple>/<mode>/` or `target/<mode>` for the default target, which in this case is target/wasm32-unknown-unknown/rele Inside that directory, we have a do_nothing.wasm file which is our “empty” Wasm
module. 
But how big is it:
`ls -lh target/wasm32-unknown-unknown/release/do_nothing.wasm`
-rwxr-xr-x 2 led staff 1.4M Jul 8 21:55 target/wasm32-unknown-unkn\ own/release/do_nothing.wasm

1.4M to do nothing! That’s insane! But it turns out it is because the output binary still includes debug symbols.

Stripping out debug symbols can be done with a tool called `wasm-strip` which is part of the WebAssembly Binary Toolkit (WABT)39.
The repository includes instructions on how to build that suite of tools. 
Assuming you have that installed somewhere on your path, we can run it to strip our binary:
`wasm-strip target/wasm32-unknown-unknown/release/do_nothing.wasm`

We can then check the size of our new binary:
`ls -lh target/wasm32-unknown-unknown/release/do_nothing.wasm`
-rwxr-xr-x 2 led staff 102B Jul 8 22:01 target/wasm32-unknown-unkn\ own/release/do_nothing.wasm

A much more reasonable 102 bytes. We can make this better by running one more tool which is to actually optimize the binary, wasm-opt as part of the Binaryen library40. Again this repository has instructions for how to build and install this suite of tools. Running this against our binary requires us to specify a new file to put the output:
`wasm-opt -o do_nothing_opt.wasm -Oz target/wasm32-unknown-unknown/relea\ se/do_nothing.wasm`

Let’s check the size of this optimized binary:
`ls -lh do_nothing_opt.wasm`
-rw-r--r--  1 led  staff    71B Jul  8 22:04 do_nothing_opt.wasm

We got down to 71 bytes. That is about as good as we can do. It is possible to manually shave a few more bytes off, 
but for all intents and purposes this is the baseline that we will build up from. 
This is incredibly small in the JavaScript ecosystem, but we are also not doing anything.
