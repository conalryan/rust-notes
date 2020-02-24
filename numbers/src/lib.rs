// Library
// Default entry point for library is lib.rs

/**
 * pub means public visibility
 * This is a privacy identifier which specifies that this function should be publicly accessible to user’s of our crate.
 * Without pub, we could call this function inside of our lib.rs file, but user’s of crate could
 * not call it.
 * Note our executable sees the library crate same as someone who included our library as a dependency in their Cargo.toml file. This ensures a proper separation of concerns between code meant to be executed as a binary and the actual functionality of your project.
 */
pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print() {
    //  Variables
    //  Immutable by default.
    //  Type is infered based on the value used to initialize the variable.
    //  Trick to see type inferred by Rust
    //  let () = numbers;
    //  type mismatch in the assignment which will print out what the compiler expects:
    //  note: expected type `[{integer}; 5]`
    //        found type `()`et numbers = [1, 2, 3, 4, 5];
    
    // Integer
    // twelve integer types (size, signed, unsigned).
    // i32 signed 32 bit interger is default. 
    // u32 is the equivalent unsigned type.
    
    // Arrays
    // Homogeneous container (all elements have the same type) with a fixed size. 
    // Stack allocated - since a homgeneous container. 
    // The ability to ensure data is stack allocated rather than heap allocated is a big benefit.
    
    // Type is inferred as i32.
    let numbers = [1, 2, 3, 4, 5]; 

    // Compiler will infer the type from the first element is used for all elements.
    let _numbers_explicit_inferred = [1u8, 2, 3, 4, 5];
    
    // Explicitly write out the type of the variable numbers:
    let _numbers_explicit_verbose: [u8; 5] = [1, 2, 3, 4, 5];
    
    // Iterator
    // Trait.
    // Abstraction for iteration. 
    // Types can implement functions that operate on themselves and can therefore be called using this dot syntax.
    // This is syntactic sugar for a direct function call with the receiver object as the first argument. 
    
    // for loop
    println!("for loop i32.iter()");
    // for variable in iterator { ... }
    //
    // We have to call iter here to turn an array into an Iterator because arrays do not automatically coerce into into an Iterator.
    // We shall see shortly that this is not always necessary with other collections.
    // Note that we are calling the method iter on our array. 
    for n in numbers.iter() {

        // Display
        // Trait.
        // “nice” format is possible when a type implements the Display trait.
        // Not all types implement Display, 
        
        // Debug
        // Trait.
        // For debugging purposes use {:?}.
        // Debugging format possible when type implements Debug trait. 
        // Standard practice all public types implement Debug. 
        // So when in doubt, use {:?} to see the value of some variable and it should almost always work.
       
        // Trait
        // Type must explicitly state it's implementing a trait in code,
        // rather than implicitly by satisfying the functional requirements of the trait. 
        // This is one of a few differences between Rust traits and Go interfaces.
        
        // print
        // Macro.
        // First argument must be a literal string, it cannot be a variable, even if that variable points to a literal string.
        // Therefore, to print out a variable you need to use the format string "{}"
        println!("{}", n);
    }

    // Arrays
    // Homogeneous container (all elements have the same type) with a fixed size. 
    // Stack allocated - since a homgeneous container. 
    // The ability to ensure data is stack allocated rather than heap allocated is a big benefit.

    // Struct std::vec::Vec
    println!("for loop Vec<T>");
    // pub struct Vec<T> { /* fields omitted */ }
    // A contiguous growable array type, written Vec<T> but pronounced 'vector'.A
    //
    // Vec<T>
    // Where T is a generic type that represents the types of the elements.
    // Vec<i32> and Vec<u8> are different types, but a Vec<u8> with four elements is the same type as one with five elements.
    // 
    // Homogeneous container (all elements have the same type) with a variable size. 
    // Similar to an array, stores single type of element in a contiguous memory block.
    // Heap allocated - the memory used by a vector is heap allocated and can therefore grow and shrink at runtime.
    // Own their data elements.
    // 
    // Length - vectors have a length which says how many elements are in the container.
    // Capacity - vectors have a capacity which could be larger than the length.
    // Changing the capacity can involve quite a bit of work to allocate a new region of memory and move all of the data into that region.
    
    // Mulitplication factor:
    // As you add elements to a vector, capacity grows by a multiplicative factor
    // Done to reduce frequency. 
    // Biggest advantage: Sive of the vector is expandable; the length is not part of the type.
    //
    // Note also that we are no longer explicitly calling iter on the numbers variable in our for loop preamble.
    // Implements a trait that tells the compiler how to convert it into an iterator in places where that is necessary like in a for loop. 
    // Calling iter explicitly would not be an error and would lead to the same running code, but this implicit conversion to an iterator is common in Rust code.
    //
    // The vec! macro is provided to make initialization more convenient:
    let number_vec = vec![1, 2, 3, 4, 5];
    for n in number_vec {
        println!("{}", n);
    }

    output_sequence(numbers);

    // output_sequence_vec(number_vec); ERROR: move occurs because `number_vec` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait
    // value moved here
    // value used here after move

    // Slices
    // A key type that comes in handy to alleviate some of the limitations of arrays is the std::slice.
    // Slices are a dynamically sized view into a sequence.
    // Therefore, you can have a slice which references an array or a vector and treat them the same.
    // This is a very common abstraction tool used in Rust.
    let vector_numbers = vec![1, 2, 3, 4, 5];
    output_sequence_vec(vector_numbers);
    // or comment line above and uncomment line below.
    // Cannot have both: Error move occurs because `vector_numbers` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait
    // output_sequence_ref(&vector_numbers);
    let array_numbers = [1, 2, 3, 4, 5];
    output_sequence_ref(&array_numbers);
}

/**
 * Different modes of passing arguments to functions.
 * Function can temporarily have acces to variable (borrowing) or have ownership of a variable.
 * Another dimension is whether the function can mutate the input.
 * Default function take input by value and hence ownership of the variable is moved into the function.
 * The exception to this rule being if the type implements a special trait called Copy, in which case the input is copied into the function and therefore the caller still maintains ownership of the variable. 
 * If the element type of an array implements the Copy trait, then the array type also implements the Copy trait.
 * While arrays implement the Copy trait if their elements do, Vec does not.
 */
fn output_sequence(numbers: [u8; 5]) {
    println!("output_sequence");
    for n in numbers.iter() {
        println!("{}", n);
    }
}

fn output_sequence_vec(numbers: Vec<u8>) {
    println!("output_sequence_vec");
    for n in numbers {
        println!("{}", n);
    }
}

fn output_sequence_ref(numbers: &[u8]) {
    println!("output_sequence_ref");
    for n in numbers {
        println!("{}", n);
    }
}
