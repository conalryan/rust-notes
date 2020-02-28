
// main function is the entry point to a binary applicatino
// It takes zero arguments and returns the empty tuple ().
// no return type is equivalent to writing -> () after the argument list of the function. 
// All function calls are expressions which must return a value. 
// The empty tuple () is a marker for no value, which is what a function with no return type implicitly returns.
fn main() {

    // Macros
    // powerful form of meta-programming 
    // you will use macros frequently but probably rarely find the occasion to have to write.
    // macros can take a variable number of arguments, while a regular function cannot.
    // macro are postfixed with !
    // e.g. println!
    // println!("Hello, world!");

    // crates can contain modules, which themselves can contain more modules.
    // syntax:
    // crate_name::module_name::item_name
    // or 
    // crate_name::item_name
    // e.g. std::thread::current
    numbers::say_hello();

    numbers::print(5);
}
