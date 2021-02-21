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
    .run()
    .await
  }
}

// http://localhost:8080/22/bob/index.html
#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}