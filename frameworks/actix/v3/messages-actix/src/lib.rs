// Note in particular that we import Result which is the type alias of Result that actix_web defines with the error type fixed to its error type.
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer, Responder, Result};
use serde::Serialize;

// Aggregate data type
// Structs
// have member data which can be of any type.
//
// Each member field has its own privacy which is not exported by default.
// You can reference instances of type MessageApp outside of our library, you cannot directly access the port field.
// We can access the port field within the file that defines the type, but otherwise it is hidden.
//
// Similar to enums, structs can also be generic over the types of data they contain.
// For example, Vec<T> which we have seen before is actually a struct called Vec which has one generic type parameter.
//
// Rust allows trailing commas in pretty much every position where a comma could exist in the future and
// it is standard practice to include them to reduce future diffs when code changes.
pub struct MessageApp {
    port: u16,
}

// Adding functionality
// impl block
//
// Rust has a strong separation of data and functionality. We defined the data representation of our struct,
// but all methods associated with the type are defined elsewhere in what is known as an impl block.
// These blocks are used for adding functionality to types as well as for implementing traits.
// All types that you create (structs, enums, etc.) can have functionality added via an impl block.
//
// A type can have multiple impl blocks associated with it, however typically there is only one main one
// with others usually only for trait implementations.
//
// Inside an impl block Self has special meaning, it refers to the type on which we are defining the implementation.
// pub fn new(port: u16) -> MessageApp is same as pub fn new(port: u16) -> Self
impl MessageApp {

  // The name of new is not special, but has become convention as the name of the constructor function for types.
  pub fn new(port: u16) -> Self {
    MessageApp { port }
  }

  // Self as parameter
  // Similar to Python where class instance methods explicitly take self as their first parameter,
  // and not taking self implies that the method is actually on the type rather than a particular instance.
  //
  // Four special first parameters:
  // - &self
  // Borrow read only access.
  // Most common form.
  // Method takes an immutable reference to the instance invoking the method.
  // We can read the data inside our type, but we cannot alter it.
  // The calling code also maintains ownership so we are just borrowing the instance.
  //
  // - &mut self
  // Borrow read and write access.
  // Second most common form.
  // Mutable version of the first form.
  // Our method can read and write the data inside our type, but it does not own the value so this access is only temporary.
  //
  // - self
  // Ownership moved into method.
  // Method consumes self and therefore the instance that the method is being called on has its ownership moved into the method.
  // This form comes usually when we are transforming a type into something else,
  // for example with interfaces that use the builder pattern.
  //
  // - mut self
  // Ownership moved into method and is mutable.
  // Method consumes self and self is mutable within the method.
  // All parameters to functions can be declared mutable if you wish them to be a mutable binding inside the function,
  // and self is no different. This has its uses, but is not that common.
  //
  // All of the forms turn a function in a method on an instance of the type.
  // This means that rather than being a function on the type which is called like
  // MessageApp::new, we must use dot syntax on an instance of the type.
  // e.g.
  // let app = MessageApp::new(8080);
  // app.run()
  pub async fn run(&self) -> std::io::Result<()> {

    let addr = format!("127.0.0.1:{}", self.port);
    println!("Starting http server:{}", addr);

    // HttpServer is the type which actix-web exposes to represent something that serves requests.
    // The constructor takes an application factory which is any function that when called returns an application.
    //
    // Closure
    // Closures in Rust can be a little tricky because of the ownership and borrowing semantics.
    // The basic syntax is to declare an argument list between pipes, ||, then
    // possibly list the return value, followed by the function body between curly braces.
    // Type inference works on closures so we can usually omit types of the arguments and return values.
    //
    // If the keyword move comes before the argument list then any variables from the environment that the closure uses
    // are actually moved into the closure.
    // This means the closure takes ownership of those variables rather than creating references.
    //
    // This implies that the lifetime of the closure can be longer can its surrounding environment because those variables are moved into the closure.
    // Without the move keyword, variables closed over are actually just references to the surrounding environment.
    //
    // Move signifies intent that the function should not have references to the environment in which it was created.
    HttpServer::new(move || {

      // Inside the closure, we are construct an App which is the abstraction actix-web defines for representing a collection of routes and their handlers.
      // new()
      // We use the new method to create an App, and then a couple methods defined on that instance to setup our application.
      //
      // wrap()
      // The wrap function wraps the app with a middleware specified as its only argument.
      // We set the Logger middleware which is provided by actix so that we can see some information about requests as they come in.
      //
      // service()
      // Furthermore, we call service(index) to specify that we want to add a service to our app which uses the handler index which we will define below.
      App::new()
        // enable logger
        .wrap(middleware::Logger::default())
        .service(index)
    })
    // ? operator
    // Common pattern of returning an error early if one occurred or otherwise pulling the value out of the Ok case and continuing on.
    // Alternative syntax without ? operator
    // let result = HttpServer::new(move || {
    //  ...
    // }).bind(("127.0.0.1", self.port));
    // if result.is_err() {
    //      return Err(result.err().unwrap());
    // }
    // result.unwrap().workers(8).run()
    .bind(addr)?
    .workers(8)
    .run()
    .await
  }
}

// http://localhost:8080/22/bob/index.html
// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

// Attributes
// Attributes are the way of attaching metadata to a variety of things in the language.
// They can be attached to modules as a whole, structs, functions, and several other constructs.
// They can attach to the thing they are defined within using the syntax #![...] with a ! after the #.

fn some_unused_variable() {
  // The allow attribute is used to turn off a lint warning for the entity that contains the
  // attribute which is the function some_unused_variable in this example.
  #![allow(unused_variables)]
  let x = ();
}


// Derived Attribute
// The derive attribute is probably the most common attribute you will encounter.
// It allows you to implement traits for types without having to do any more work provided
// the type meets the requirements for the trait to be derived.
//
// Most structs will at the very least will derive Debug which allows the struct to be printed
// using the {:?} debug format specifier.
// Note all builtin types implement Debug trait.
//
// Now that we have derived Serialize any instance of our struct can be serialized by serde into
// the output format of our choice.
#[derive(Serialize)]
struct IndexResponse {
  message: String,
}

// Handlers in Rust
// Most of the work in defining a handler in all of the Rust web ecosystem is centered around defining
// the input and output types. Current idiomatic design focuses on the
// type signature explaining what the function uses.
//
// The alternative would be handlers that all
// take a generic request as input and return generic response as output
// and then the internals of the function need to be introspected to determine what a handler does.
//
// curl localhost:8080
// curl -H "hello:actix" localhost:8080
#[get("/")]
async fn index(req:HttpRequest) -> Result<web::Json<IndexResponse>> {

  // Working with Options
  // Option<T> is an enum in the standard library with two variants: Some(T) and None.
  //
  // The idea of Option is to represent the possibility of something not always existing and
  // hence replaces the need for the concept of null found in many other programming languages.
  // The major distinction between null in other languages and Option in Rust is that an Option
  // is an explicit type that has a None variant that you must deal with and thus the concept of
  // null cannot inhabit other types.
  //
  // In many other languages null can be the value of nearly every type of variable.
  // Option is the other main error handling primitive that complements Result.
  // Wherein Result carries an error value, sometimes you either have something or you don’t
  // and in those scenarios Option is the more suitable type to use
  //
  // headers.get("hello") will return an  Option<&HeaderValue>
  let hello = req
      .headers()
      .get("hello")
      // and_then is a no-op on None
      .and_then(|v| v.to_str().ok())
      .unwrap_or_else(|| "world");

  Ok(web::Json(IndexResponse {
      message: hello.to_owned(),
  }))
}


// String
// Most primitive string type is named str and is known as a string slice.
// This is a slice in the same sense that [i32] is a slice of signed 32-bit integers.
// A string slice is a slice of bytes, i.e. it has type [u8] and it also is valid Unicode.
//
// The str type is almost always encountered as the borrowed variant &str which is a reference to a valid Unicode byte array.
// The reference means that it points to memory owned by someone else.
// In particular, static string literals are represented with type &'static str
// where the notation &'static means a reference to something with a static lifetime.
// The static lifetime is a special lifetime in Rust which is the entire life of your program.
// Static strings are compiled into your binary and are therefore owned by the binary.
// The other type of string has type String which is a heap allocated string, i.e. it is a string you own.
