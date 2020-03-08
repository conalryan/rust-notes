use wasm_bindgen::prelude::*;

// Then getting your code exposed to JavaScript is as simple as adding the wasm_bindgen attribute. 
// Our greet function needs to be public to be exposed, but otherwise this is all the code we need to write.
//
// Now we can use wasm-pack to build a JavaScript package that contains our compiled Wasm code with one simple command:
// wasm-pack build
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
