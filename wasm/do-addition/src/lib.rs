// We expose a single function add which takes two unsigned integers and returns their sum as an unsigned integer:
//
// #[no_mangle] attribute tells the Rust compiler that we want the name of our function to be 
// add in the final binary instead of some more complicated name that is auto-generated based on the name and types.
//
// Both Rust and C++ use name mangling for managing certain language features that are easier to implement 
// if everything has a unique name.
// Usually you don’t have to worry about the exact name of your functions in the compiled executable, 
// but because we are exposing a library which will be callable from JavaScript we need to know the actual name we need to call. 
// Without this attribute, we would end up with something like N15do_addition_4a3b56d3add3 as the name of the add function.
//
// We also put the modifier extern "C" on the function to say that we want this function 
// to use the right calling conventions that Wasm will understand. 
// Otherwise this is just a simple publicly exposed Rust function.
#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}
