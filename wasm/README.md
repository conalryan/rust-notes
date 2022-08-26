[Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
========================================================================================================================

[1. Introduction](https://rustwasm.github.io/docs/book/introduction.html)
========================================================================================================================

Target audience is compiling Rust to WebAssembly for fast, reliable code on the Web.

[2. Why Rust and WebAssembly?](https://rustwasm.github.io/docs/book/why-rust-and-webassembly.html)
========================================================================================================================

[Low-Level Control with High-Level Ergonomics](https://rustwasm.github.io/docs/book/why-rust-and-webassembly.html#low-level-control-with-high-level-ergonomics)
------------------------------------------------------------------------------------------------------------------------

JavaScript Web applications struggle to attain and retain reliable performance.
JavaScript's dynamic type system and garbage collection pauses don't help.
Seemingly small code changes can result in drastic performance regressions if you accidentally wander off the JIT's happy path.A

[With Rust] Programmers have control over indirection, monomorphization, and memory layout.

Incremental Adoption
------------------------------------------------------------------------------------------------------------------------

You can start by porting your most performance-sensitive JavaScript functions to Rust to gain immediate benefits.

Plays Well With Others
------------------------------------------------------------------------------------------------------------------------

Rust and WebAssembly integrates with existing JavaScript tooling.
- Supports ECMAScript modules
- Supports npm
- Supports Webpack

[3. What is WebAssembly](https://rustwasm.github.io/docs/book/what-is-webassembly.html)
========================================================================================================================

WebAssembly (wasm) is a simple machine model and executable format with an extensive specification. It is designed to be portable, compact, and execute at or near native speeds.

As a programming language, WebAssembly is comprised of two formats that represent the same structures, albeit in different ways:
1. The .wat text format (called wat for "WebAssembly Text") uses S-expressions, and bears some resemblance to the Lisp family of languages like Scheme and Clojure.
2. The .wasm binary format is lower-level and intended for consumption directly by wasm virtual machines. It is conceptually similar to ELF and Mach-O.

[wat2wasm](https://webassembly.github.io/wabt/demo/wat2wasm/)

[Linear Memory](https://rustwasm.github.io/docs/book/what-is-webassembly.html#linear-memory)
------------------------------------------------------------------------------------------------------------------------

WebAssembly has a very simple memory model.
A wasm module has access to a single "linear memory", which is essentially a flat array of bytes.
This memory can be grown by a multiple of the page size (64K).
It cannot be shrunk.

Is WebAssembly Just for the Web?
------------------------------------------------------------------------------------------------------------------------

wasm makes no assumptions about its host environment.
wasm might become a "portable executable" format that is used in a variety of contexts in the future.

[4. Tutorial](./tutorial/README.md)
========================================================================================================================

