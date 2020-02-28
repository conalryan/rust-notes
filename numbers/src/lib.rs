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

pub fn print(limit: u8) {
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
    // 
    // Mulitplication factor:
    // As you add elements to a vector, capacity grows by a multiplicative factor
    // Done to reduce frequency. 
    // Biggest advantage: Sive of the vector is expandable; the length is not part of the type.
    //
    // Copy Trait
    // While arrays implement the Copy trait if their elements do, Vec does not.
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
    // output_sequence_vec(vector_numbers);
    // or uncomment line above and comment line below.
    // Cannot have both: Error move occurs because `vector_numbers` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait
    output_sequence_ref(&vector_numbers);
    let array_numbers = [1, 2, 3, 4, 5];
    output_sequence_ref(&array_numbers);

    let numbers_seq = generate_sequence(limit);
    output_sequence_ref(&numbers_seq);

    let numbers_seq_collect = generate_sequence_collect(limit);
    output_sequence_ref(&numbers_seq_collect);
}

/**
 * Functions
 * Inputs passed by value.
 * Therefore, ownership is also moved into the function.
 *
 * Modes: Different modes of passing arguments to functions.
 * Borrowing: 
 *  - Function can temporarily have acces to variable (borrowing).
 * Ownership:
 *  - Functions can have ownership of a variable.
 *  - Default function take input by value and hence ownership of the variable is moved into the function.
 * 
 * Another dimension is whether the function can mutate the input.
 * 
 * Copy
 * Trait
 * If type implements a special trait called Copy, input is copied into the function and therefore the caller still maintains ownership of the variable. 
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

/**
 * A type signature that works for both arrays and vectors
 * [u8] slice of u8 values. 
 * Unknown size at compile time. 
 * Functions cannot take arguments of an unknown size. 
 * 
 * Indirection
 * Allows access to slice of unknown size by passing a reference to the slice.
 * &[u8] reference to a slice of u8 values which has a known size at compile time.
 *
 * Size is equal to size of the pointer plus the length of the slice,
 * therefore, it is know at compile time.
 *
 * Note slices convert automatically into iterators just like vectors, therefore no call to iter().
 *
 * & before variable name creates a slice that represents read-only access to the entire sequence for both the vector and array.
 * Idiomatic Rust takes slices as arguments in most cases where one needs only to read the collection.
 * This is particularly true for strings which we will cover later.
 *
 * The major difference here is that we are no longer transferring ownership into the function output_sequence instead we are lending read-only access to that function.
 * The data is only borrowed for the duration of the function call.
 */
fn output_sequence_ref(numbers: &[u8]) {
    println!("output_sequence_ref");
    for n in numbers {
        println!("{}", n);
    }
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    // Unlike in some other languages, new is not special but rather has become by convention 
    // the name of the function that returns a new instance of a type. 
    //
    // You can write a function called new which does something else and it would compile just fine,
    // but it would go against the standard way of doing things.
    //
    // By default a vector created with new, is the same as one created with vec![], and does not allocate.
    // Therefore, unless you actually put something into a vector it does not use any memory.
    //
    // Mutability is a property of the variable or reference not of the object itself.
    let mut numbers = Vec::new();

    // iterator is a Range object, in particular an InclusiveRange. 
    // Ranges can be constructed with using the syntax
    // 
    // Inclusive start, exclusive end
    // start..end
    // let numbers = [1, 2, 3, 4, 5]; 
    // let subset = &numbers[1..3]; // 2, 3
    //
    // Inclusive start, inclusive end use =end
    // start..=end.
    for n in 1..=limit {
        // Iterating over this range, we push each value onto the end of our vector which causes 
        // heap allocations every time there is not enough capacity to extend the length.
        numbers.push(n);
    }
    // The final expression in a function is implicitly returned so there is no need for an explicit return statement
    // The expression that evaluates to the vector numbers is written without a semicolon and means to return that value.
    // If we had written a semicolon, that would be a statement whose value is () which is not what you want to return.
    // This is a common error so the compiler is smart enough to tell you what to fix, but it is nonetheless an error. 
    // You can use a return statement to return early from a function, but using the last expression of the block as the implicit return is idiomatic Rust.
    numbers
}

// A Shorter Version with collect
// This function can be used to turn any iterator into basically any collection.
fn generate_sequence_collect(limit: u8) -> Vec<u8> {
    // Collect is a generic function over the return type, so the caller gets to determine what they want. 
    // Here because we return the result of calling collect from our function, type inference sees that the return type needs to be a Vec<u8> and therefore ensures that collect generates that collection.
    // turn one collection into another collection
    // turning a range into a vector
    //
    // alterantive syntax ::<>, you may see referred to as the “turbofish”
    // collect::<SomeType>()
    (1..=limit).collect()
}

// Test
// Test is just a normal function with a special attribute, #[test], before it.
// They come in two forms #[...] and #![...] which annotate the item they precede.
#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1, 2, 3]);
}
