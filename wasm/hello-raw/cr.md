Working with complex types
--------------------------------------------------------------------------------
Being limited to integers and floating point numbers might seem like a pretty severe limitation, 
but any complex type can be built on top of these primitives with some effort and hopefully help from some tools.

`cargo new --lib hello-raw`

Add to `Cargo.toml`
`
[lib]
crate-type=["cdylib"]
`

The Real Way of Write Wasm
--------------------------------------------------------------------------------

The majority of the heavy lifting is done by the [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) crate and CLI. 
Built on top of wasm-bindgen is the [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html) tool. 
This is a build tool which automates the process of exposing your Wasm module as an NPM module. 
The wasm-pack documentation has examples and templates for getting up and running with Wasm in the browser and in NodeJS environments. 
We are going to replicate our simple greet function but this time rely on these tools to do the work.

