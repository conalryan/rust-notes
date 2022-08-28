[Rust and WebAssembly Tutorial](https://rustwasm.github.io/docs/book/)

[4. Tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
========================================================================================================================

[4.1 Setup](https://rustwasm.github.io/docs/book/game-of-life/setup.html)
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

[4.2 Hello, World](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
------------------------------------------------------------------------------------------------------------------------

`cargo generate --git https://github.com/rustwasm/wasm-pack-template`

This should prompt you for the new project's name. We will use "wasm-game-of-life".

`wasm-game-of-life`

Cargo.toml will now have
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2.63"

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.6", optional = true }

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. It is slower than the default
# allocator, however.
#
# Unfortunately, `wee_alloc` requires nightly Rust when targeting wasm for now.
wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

### [Putting it into a webpage](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#putting-it-into-a-web-page)

To take our `wasm-game-of-life` package and use it in a Web page, we use the `create-wasm-app` JavaScript project template.

Run this command within the wasm-game-of-life directory:

`npm init wasm-app www`

`cd www && npm install`

### [Using your local `wasm-game-of-life` Package in `www`](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#using-our-local-wasm-game-of-life-package-in-www)

Rather than use the `hello-wasm-pack` package from npm, we want to use our local `wasm-game-of-life` package instead.
This will allow us to incrementally develop our Game of Life program.

Open up `wasm-game-of-life/www/package.json` and next to `"devDependencies"`, add the `"dependencies"` field, including a `"wasm-game-of-life": "file:../pkg"` entry:
```json
{
  // ...
  "dependencies": {                     // Add this three lines block!
    "wasm-game-of-life": "file:../pkg"
  },
  "devDependencies": {
    //...
  }
}
```

Next, modify `wasm-game-of-life/www/index.js` to import `wasm-game-of-life` instead of the `hello-wasm-pack` package:

```javascript
import * as wasm from "wasm-game-of-life";

wasm.greet();
```

Since we declared a new dependency, we need to install it:

`npm install`

`npm run start`


[4.4 Interfacing Rust and Javascript](https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript)
------------------------------------------------------------------------------------------------------------------------

wasm_bindgen defines a common understanding of how to work with compound structures across this boundary.
It involves boxing Rust structures, and wrapping the pointer in a JavaScript class for usability, or indexing into a table of JavaScript objects from Rust.
wasm_bindgen is very convenient, but it does not remove the need to consider our data representation, and what values and structures are passed across this boundary.
Instead, think of it as a tool for implementing the interface design you choose.

When designing an interface between WebAssembly and JavaScript, we want to optimize for the following properties:

    Minimizing copying into and out of the WebAssembly linear memory. Unnecessary copies impose unnecessary overhead.

    Minimizing serializing and deserializing. Similar to copies, serializing and deserializing also imposes overhead, and often imposes copying as well.
    If we can pass opaque handles to a data structure — instead of serializing it on one side, copying it into some known location in the WebAssembly linear memory, and deserializing on the other side — we can often reduce a lot of overhead. wasm_bindgen helps us define and work with opaque handles to JavaScript Objects or boxed Rust structures.

As a general rule of thumb, a good JavaScript↔WebAssembly interface design is often one where large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory, and are exposed to JavaScript as opaque handles. JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result. By only returning the small result of the computation, we avoid copying and/or serializing everything back and forth between the JavaScript garbage-collected heap and the WebAssembly linear memory.

cr. WIP https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript-in-our-game-of-life
