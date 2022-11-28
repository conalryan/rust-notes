// Traits: Defining Shared Behavior
// A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.
// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
//
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
// Declared the trait as pub so that crates depending on this crate can make use of this trait too.
// Each type implementing this trait must provide its own custom behavior for the body of the method.
// The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
// A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.
pub trait Summary {
    fn summarize(&self) -> String;
}

// Default implementation
// To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with impl Summary for NewsArticle {}.
// impl SummaryWithDefault for NewsArticle {}.
pub trait SummaryWithDefault {
    fn summarize_default(&self) -> String {
        String::from("default summary")
    }
}

// Default implementation can call other traits
// You only need to define summarize_author on a type to satifsy compiler
pub trait SummaryMixed {
    fn summarize_author(&self) -> String;

    // Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
    // In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.
    fn summarize_mixed(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing a trait on a type is similar to implementing regular methods.
// The difference is that after impl, we put the trait name we want to implement, then use the for keyword,
// and then specify the name of the type we want to implement the trait for.
//
// One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
// We can implement standard library traits like Display on a custom type
// We can implement custom traits for standard types i.e. We can also implement Summary on Vec<T> in our aggregator crate,
// because the trait Summary is local to our aggregator crate.
//
// But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our crate,
// because Display and Vec<T> are both defined in the standard library and aren’t local to our crate.
//
// This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present.
// This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type,
// and Rust wouldn’t know which implementation to use.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with impl Summary for NewsArticle {}.
impl SummaryWithDefault for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryMixed for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
// --------------------
// impl Trait syntax
// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
// This parameter accepts any type that implements the specified trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
// ------------------
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax is better for functions with multiple parameters
pub fn notify_multiple_impl_trait(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify_multiple_trait_bound<T: Summary>(item1: &T, item2: &T) {}

// Specifying Multiple Trait Bounds with the + Syntax
use std::fmt::Display;

pub fn notify_multiple_types_impl_trait(item: &(impl Summary + Display)) {}
pub fn notify_multiple_types_trait_bound<T: Summary + Display>(item: &T) {}

// where clauses
// -------------
use std::fmt::Debug;

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  21
}

fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
  22
}

// Returning Types that Implement Traits
// -------------------------------------
// We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.
// The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators.
// Closures and iterators create types that only the compiler knows or types that are very long to specify.
// The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.
// Howev ever you can only use impl Trait if you’re returning a single type.
// i.e. Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler.
fn returns_summarizable() -> impl Summary {
  Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
          "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
  }
}

// Using Trait Bounds to Conditionally Implement Methods
// -----------------------------------------------------

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
// But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison
// and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
// and are extensively used in the Rust standard library.
// For example, the standard library implements the ToString trait on any type that implements the Display trait.
// The impl block in the standard library looks similar to this code:
// impl<T: Display> ToString for T {
//     // --snip--
// }