// https://doc.rust-lang.org/book/ch10-00-generics.html

mod lifetimes;
mod traits;

use traits::{NewsArticle, Summary, SummaryMixed, SummaryWithDefault, Tweet};

fn main() {
    println!("\n Generics Traits and Lifetimes");

    // Rust’s type-naming convention is CamelCase.
    // Short for “type,” T is the default choice of most Rust programmers.

    // Generic Function
    // ----------------
    // when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.
    // To define the generic largest function, place type name declarations inside angle brackets, <>,
    // between the name of the function and the parameter list.
    // To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
    // use std::cmp::PartialOrd;

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest { // error[E0369]: binary operation `>` cannot be applied to type `&T`
                // consider restricting type parameter `T`: fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
                largest = item;
            }
        }

        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // Generic Struct
    // --------------
    // The syntax for using generics in struct definitions is similar to that used in function definitions.
    // First, we declare the name of the type parameter inside angle brackets just after the name of the struct.
    // Then we use the generic type in the struct definition where we would otherwise specify concrete data types.
    //
    // To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.
    struct Point<T, U> {
        x: T,
        y: U,
    }


    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    // In Method Definitions
    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Constraints on generic type methods
    impl Point<f32, f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let distance = both_float.distance_from_origin();
    println!("distance is {distance}");

    // Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
    impl<T1, U1> Point<T1, U1> {
        fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T1, U2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Enums can use Generics
    // ---------------------
    enum Option<T> {
        Some(T),
        None,
    }

    // Enums can use multiple generics
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Traits
    // ------
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize_mixed());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Default article: {}", article.summarize_default());


    // Lifetimes
    // ---------
    // The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
    lifetimes::wont_compile();

    lifetimes::static_lifetime()
}

