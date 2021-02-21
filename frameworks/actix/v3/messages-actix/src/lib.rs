use actix_web::Responder;
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer, Result};
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
  // Most common form. This means that our method takes an immutable reference to the instance invoking the method.
  // We can read the data inside our type, but we cannot alter it.
  // The calling code also maintains ownership so we are just borrowing the instance.
  //
  // - self
  // Method consumes self and therefore the instance that the method is being called on has its ownership moved into the method.
  // This form comes usually when we are transforming a type into something else,
  // for example with interfaces that use the builder pattern.
  //
  // - &mut self
  //  Mutable version of the first form. This is the second most common thing you will encounter.
  //  Our method can read and write the data inside our type, but it does not own the value so this access is only temporary.
  //
  // - mut self
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
    println!("Starting http server: 127.0.0.1:{}", self.port);

    HttpServer::new(||
      App::new()
        .service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
  }
}

// http://localhost:8080/22/bob/index.html
#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}