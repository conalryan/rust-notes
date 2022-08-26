[Tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
========================================================================================================================

[Setup](https://rustwasm.github.io/docs/book/game-of-life/setup.html)
------------------------------------------------------------------------------------------------------------------------

This section describes how to set up the toolchain for compiling Rust programs to WebAssembly and integrate them into JavaScript.

### The Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and `cargo`.

Rust 1.30 or newer.

### wasm-pack

`cargo install wasm-pack`

`wasm-pack` is your one-stop shop for building, testing, and publishing Rust-generated WebAssembly.

### cargo-generate

cargo-generate helps you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

Install `cargo-generate` with this command:

`cargo install cargo-generate`

### npm

`npm` is a package manager for JavaScript.
We will use it to install and run a JavaScript bundler and development server.
At the end of the tutorial, we will publish our compiled `.wasm` to the npm registry.

If you already have npm installed, make sure it is up to date with this command:

`npm install npm@latest -g`

