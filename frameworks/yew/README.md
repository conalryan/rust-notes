# [Yew](https://yew.rs/)

Yew is a modern Rust framework for creating multi-threaded front-end web apps using WebAssembly.

- component-based framework similar to React and Elm.
- Great performance by minimizing DOM API calls and offload processing to background threads using web workers.
- JavaScript interoperability, leverage NPM packages and integrate with existing JavaScript applications.

## Install WebAssembly target

Rust can compile source codes for different "targets" (e.g. different processors).
The compilation target for browser-based WebAssembly is called "wasm32-unknown-unknown". 

```bash
rustup target add wasm32-unknown-unknown
```

## Install Trunk

Trunk is the recommended tool for managing deployment and packaging, and will be used throughout the documentation and examples.
See **Wasm Build Tools** for more information on packaging and alternatives.

```bash
# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install trunk
```

