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

## Build Tools

### Install Trunk

Trunk is the recommended tool for managing deployment and packaging, and will be used throughout the documentation and examples.
See **Wasm Build Tools** for more information on packaging and alternatives.

```bash
# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install trunk
```

### Wasm-pack

## Concepts

### [wasm-bindgen](https://yew.rs/docs/concepts/wasm-bindgen)

wasm-bindgen is a library and tool to facilitate high-level interactions between Wasm modules and JavaScript.

Yew is built on wasm-bindgen and specifically uses the following of its crates:

- js-sys
- wasm-bindgen
- wasm-bindgen-futures
- web-sys

#### [\#\[wasm-bindgen\] macro](https://yew.rs/docs/concepts/wasm-bindgen#wasm_bindgen-macro)

The `#[wasm_bindgen]` macro, in a high level view, is your translator between Rust and JavaScript.
It allows you to describe imported JavaScript types in terms of Rust and vice versa. 
Using this macro is more advanced, and you shouldn't need to reach for it unless you are trying to interop with an external JavaScript library. 
The `js-sys` and `web-sys` crates are essentially imported types using this macro for JavaScript types and the browser API respectively.

#### [JsValue](https://yew.rs/docs/concepts/wasm-bindgen#jsvalue)

This is a representation of an object owned by JavaScript, this is a root catch-all type for wasm-bindgen. Any type that comes from wasm-bindgen is a JsValue.

When you are working with imported functions or types that accept a JsValue, then any imported value is technically valid.

JsValue can be accepted by a function but that function may still only expect certain types and this can lead to panics - so when using raw wasm-bindgen APIs check the documentation of the JavaScript being imported whether an exception will be caused if that value is not a certain type.

#### [JsCast](https://yew.rs/docs/concepts/wasm-bindgen#jscast)

Move from one JavaScript "type" to another.

#### [Closure](https://yew.rs/docs/concepts/wasm-bindgen#closure)

Transfer Rust closures to JavaScript, the closures past to JavaScript must have a 'static lifetime for soundness reasons.

#### [js-sys](https://yew.rs/docs/concepts/wasm-bindgen#js-sys)

Provides bindings / imports of JavaScript's standard, built-in objects, including their methods and properties.

This does not include any web APIs as this is what web-sys is for!

#### [wasm-bindgen-futures](https://yew.rs/docs/concepts/wasm-bindgen#wasm-bindgen-futures)

The wasm-bindgen-futures crate provides a bridge for working with JavaScript Promise types as a Rust Future, and similarly contains utilities to turn a rust Future into a JavaScript Promise. 
This can be useful when working with asynchronous or otherwise blocking work in Rust (wasm), and provides the ability to interoperate with JavaScript events and JavaScript I/O primitives.

### [web-sys](https://yew.rs/docs/concepts/wasm-bindgen/web-sys)

Provides bindings for Web APIs.

This is procedurally generated from browser WebIDL which is why some of the names are so long and why some of the types are vague.

Use `Deref::deref` on an HtmlElement until you get `JsValue`.

Most types are feature gated so that you only include the types you require for your application.

There are three main interfaces in this crate currently:

1. `JsFuture` - A type that is constructed with a Promise and can then be used as a `Future<Output=Result<JsValue, JsValue>>`. 
This Rust future will resolve or reject with the value coming out of the Promise.

2. `future_to_promise` - Converts a Rust `Future<Output=Result<JsValue, JsValue>>` into a JavaScript Promise. 
The future’s result will translate to either a resolved or rejected Promise in JavaScript.

3. `spawn_local` - Spawns a `Future<Output = ()>` on the current thread. This is the best way to run a Future in Rust without sending it to JavaScript.

#### [spawn_local](https://yew.rs/docs/concepts/wasm-bindgen#spawn_local)A

Most commonly used part of the wasm-bindgen-futures crate in Yew as this helps when using libraries that have async APIs.

### [Components](https://yew.rs/docs/concepts/components)

- Building blocks of Yew.
- Manage own state and can render themselves to the DOM.
- Created by implementing the `Component` trait for a type.
    - Component trait has a number of methods which need to be implemented.
    - Yew will call these methods at different stages in the lifecycle of a component.

#### [Create](https://yew.rs/docs/concepts/components/introduction#create)

```rust
/// Called on component creation.
/// Receives &Context<Self> which contains:
/// - properties passed down from parent that can be used to initialize component's state.
/// - link that can be used to register callbacks or send messages to the component.
fn create(ctx: &Context<Self>) -> Self {
    MyComponent
}
```


```rust
pub struct SomeModel {
    pub some_prop: AttrValue,
}

pub struct SomeComponent;

impl Component for SomeComponent {
    type Message = ();
    type Properties = SomeModel;

    fn create(ctx: &Context<Self>) -> {
        let some_prop = ctx.props().some_prop.clone();

        Self {
            some_prop,
        }
    }
}
```

```rust
use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        MyComponent
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl
        }
    }
}
```

