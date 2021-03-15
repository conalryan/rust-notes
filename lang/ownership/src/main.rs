fn main() {
    let s_literal = "Hello";
    println!("{}, world!", s_literal);

    // String type is alocalted on the heap
    let s_from = String::from("hello");
    println!("{}, world!", s_from);

    // String can be mutated
    // Why can String be mutated but literals cannot? The difference is how these two types deal with memory.
    // string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.
    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
    // Memory management
    // We need to pair exactly one allocate with exactly one free.
    // drop
    // There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
    //
    // Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.
    //
    let mut s_mut_from = String::from("hello");
    s_mut_from.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s_mut_from); // This will print `hello, world!`

    // Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But Figure 4-2 shows both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    // To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:
    // instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2
    // Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    let s_owner_one = String::from("hello");
    let s_owner_two = s_owner_one;

    // uncomment to see error println!("{}, world!", s_owner_one); // borrow of moved value
    println!("{}, world!", s_owner_two);

    let s_owner_one = String::from("hello");
    let s_owner_two = s_owner_one.clone();

    println!("s1 = {}, s2 = {}", s_owner_one, s_owner_two); // no error because it's not a borrow it's a copy


    // variables stored entirely on the stack can be copied because it's so cheap
    // therefore no need to call clone like with heap allocated objects.
    // copy trait
    // general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:
    //
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // move
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here

    // copy
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward

    // move
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    // ping pong move
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    // References &
    // These ampersands are references, and they allow you to refer to some value without taking ownership of it.
    // We call having references as function parameters borrowing.
    //
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // uncomment to see error let r2 = &mut s; // cannot borrow more than once
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

}
// Here, end of scope (end of block)
// x goes out of scope
// s goes out of scope but because s's value was moved, nothing special happens.
// s3 goes out of scope and is dropped
// s2 goes out of scope but was moved, so nothing happens
// s1 goes out of scope and is dropped.


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}


// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}


// References
// We call having references as function parameters borrowing.
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.