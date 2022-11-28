// Lifetimes
// ---------
// lifetimes ensure that references are valid as long as we need them to be.
// every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
// We only must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes
// when the lifetimes of references could be related in a few different ways.
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual
// references used at runtime will definitely be valid.
//
// Preventing Dangling References with Lifetimes
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data
// other than the data it’s intended to reference.
pub fn wont_compile() {
    let r = 22;

    {
        let x = 5;
        // r = &x; // error[E0597]: `x` does not live long enough
    }

    println!("r: {}", r);
}

// Lifetime Annotation Syntax
// --------------------------
// Lifetime annotations don’t change how long any of the references live.
// Rather, they describe the relationships of the lifetimes of multiple references to each other without
//  affecting the lifetimes.
// Just as functions can accept any type when the signature specifies a generic type parameter,
// functions can accept references with any lifetime by specifying a generic lifetime parameter.
//
// Lifetime annotations:
// the names of lifetime parameters must start with an apostrophe (')
// and are usually all lowercase and very short, like generic types.
// Most people use the name 'a for the first lifetime annotation.
// We place lifetime parameter annotations after the & of a reference, using a space to separate the
// annotation from the reference’s type.
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
//
// One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust
// how generic lifetime parameters of multiple references relate to each other.

// The Borrow Checker
// ------------------
// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
// In the above example the variable "r" lives longer than the variable "x."
// All variables are assigned a lifetime by the borrow checker.
// Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure
// references will always be valid, let’s explore generic lifetimes of parameters and return values in the
// context of functions.
pub fn run() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}

// Lifetime Annotations in Function Signatures
// -------------------------------------------
// To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters
// inside angle brackets
// between the function name and the parameter list, just as we did with generic type parameters.
// In practice, it means that the lifetime of the reference returned by the longest function is
// the same as the smaller of the lifetimes of the values referred to by the function arguments.
//
// When annotating lifetimes in functions, the annotations go in the function signature, not in the function
//  body.
// The lifetime annotations become part of the contract of the function, much like the types in the signature.
//
// When returning a reference from a function, the lifetime parameter for the return type needs to match the
//  lifetime parameter for one of the parameters.
// If the lifteimes don't work the best fix would be to return an owned data type rather than a reference so
//  the calling function is then responsible for cleaning up the value.
//
// Note that we want the function to take string slices, which are references, rather than strings,
// because we don’t want the longest function to take ownership of its parameters.
// Without lifetime annotations, the borrow checker can’t determine how the lifetimes of x and y relate to
// the lifetime of the return value.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}


fn longest_with_lifetimes() {
  let string1 = String::from("long string is long");
  {
      let string2 = String::from("xyz");
      let result = longest(string1.as_str(), string2.as_str());
      println!("The longest string is {}", result);
  }
}

fn longest_with_lifetimes_fail() {
  let string1 = String::from("long string is long");
  let result = String::from("failed");
  {
      let string2 = String::from("xyz");
      // result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {}", result);
}

// Lifetime Annotations in Struct Definitions
// ------------------------------------------
// So far, the structs we’ve defined all hold owned types. We can define structs to hold references,
// but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
// This annotation means an instance of the strcut can’t outlive the reference it holds.
struct ImportantExcerpt<'a> {
  part: &'a str,
}

pub fn run_struct_lifetime() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
      part: first_sentence,
  };
}

// Lifetime Elision
// ----------------
// Historical considertions: in early versions (pre-1.0) of Rust,
// `fn first_word(s: &str) -> &str { ... }`
// wouldn’t have compiled because every reference needed an explicit lifetime.
// At that time, the function signature would have been written like this:
// `fn first_word<'a>(s: &'a str) -> &'a str { ... }`
//
// After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same
// lifetime annotations over
// and over in particular situations. These situations were predictable and followed a few deterministic
// patterns.
// The developers programmed these patterns into the compiler’s code so the borrow checker could infer the
// lifetimes in these situations and wouldn’t need explicit annotations.
// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
//
// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values
// are called output lifetimes.
//
// The compiler uses three rules to figure out the lifetimes of the references when there
// aren’t explicit annotations.
// The first rule applies to input lifetimes, and the second and third rules apply to output
// lifetimes.
// If the compiler gets to the end of the three rules and there are still references for which
// it can’t figure out lifetimes,
// the compiler will stop with an error. These rules apply to fn definitions as well as impl
// blocks
//
// Rule 1
// Is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
// In other words, a function with one parameter gets one lifetime parameter:
// fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters:
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
//
// Rule 2
// If there is exactly one input lifetime parameter, that lifetime is assigned to all output
// lifetime parameters:
// fn foo<'a>(x: &'a i32) -> &'a i32.
//
// Rule 3
// If there are multiple input lifetime parameters, but one of them is &self or &mut self
//  because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.

// Lifetime Annotations in Method Definitions
// ------------------------------------------
// Lifetime names for struct fields always need to be declared after the impl keyword
// and then used after the struct’s name,
// because those lifetimes are part of the struct’s type.
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
      3
  }
}

// Here is an example where the third lifetime elision rule applies:
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The Static Lifetime
// -------------------
// One special lifetime we need to discuss is 'static, which denotes that the affected reference
// can live for the entire duration of the program.
// All string literals have the 'static lifetime, which we can annotate as follows:
pub fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
    println!("s is {s}");
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
// -------------------------------------------------------------
use std::fmt::Display;

// Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a
// and the generic type parameter T go in the same list inside the angle brackets after
// the function name.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}