// edit index.js to load our package and call the greet function:
// hello-bindgen-app/index.js
import * as wasm from "hello-bindgen"; 
let result = wasm.greet("Rust");
console.log(result);
