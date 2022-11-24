# [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)

Enumerations, also referred to as enums. Enums allow you to define a type by enumerating its possible variants.

enums give you a way of saying a value is one of a possible set of values.

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two

## [Option enum]()

Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

    I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the standard library as follows:
```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `match` expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

Patterns can be made up of literal values, variable names, wildcards, and many other things.

Think of a match expression as being like a coin-sorting machine

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run.

We don’t typically use curly brackets if the match arm code is short, as it is in Listing 6-3 where each arm just returns a value. If you want to run multiple lines of code in a match arm, you must use curly brackets, and the comma following the arm is then optional. For example, the following code prints “Lucky penny!” every time the method is called with a Coin::Penny, but still returns the last value of the block, 1:
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Matches Are Exhaustive

There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities.

error[E0004]: non-exhaustive patterns: `None` not covered
