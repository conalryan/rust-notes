use actix_web::{get, middleware, web, App, HttpRequest, HttpServer, Responder, Result};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// Static versus const
// Both live for entirety of program.
// Both static and const variables must have their types given explicitly
//
// const
// Items marked with const are effectively inlined at each site they are used.
// Therefore references to the same constant do not necessarily point to the same memory address.
//
// static
// static items are not inlined, they have a fixed address as there is only one instance for each value.
// Hence static must be used for a shared global variable.
//
// Actix by default will create a number of workers to enable handling concurrent requests.
// One piece of state we are going to maintain is a unique usize for each worker.
// We will create an atomic usize to track this count of workers because it needs to be thread-safe
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Each worker thread gets its own instance of this state struct.
// Actix takes an application factory because it will create many instances of the application,
// and therefore many instances of the state struct.
//
// Rust has a pattern for mutating a piece of data inside a struct which itself is immutable known as interior mutability.
// Two special types enable this:
// - Cell
// Cell implements interior mutability by moving values in and out of a shared memory location.
// - RefCell
// RefCell implements interior mutability by using borrow checking at runtime to enforce the constraint
// that only one mutable reference can be live at any given time.
// If one tries to mutably borrow a RefCell that is already mutably borrowed the calling thread will panic.
//
// Cell and RefCell are not needed that often in everyday Rust
struct AppState {
  server_id: usize,
  request_count: Cell<usize>,
  // We can ensure mutually exclusive access to the vector by creating a Mutex that wraps our vector.
  //
  // Typically each value in Rust has a single owner, but for this situation we want each thread to be an owner of the data
  // so that the vector lives until the last worker thread exits.
  // The mechanism for this in Rust is to use a reference counted pointer. There are two variants:
  // - Rc<T>
  // Calling clone on an Rc will produce a new pointer to the same value on the heap.
  // When the last Rc pointer to a value is destroyed, the pointed-to value will then be destroyed.
  // Rc is non-atomic and therefore not thread safe.
  // You cannot share Rc across threads.
  // Uses a trait called Deref to alow you toa call the methods of T directily on a value of type Rc<T>
  // As Rust does not have a garbage collector, it is possible to create memory leaks by creating cycles of reference counted pointers.
  // There is a non-owning variant called Weak which can be used to break such cycles.
  // This is not an issue for us here, but it is important to be aware of especially if you are coming from a garbage collected language.
  // - Arc
  // A in arc stands for atomic as the reference counting mecahnism.
  // They both are generic over a type T and provide a reference counted pointer to a value of type T allocated on the heap.
  // You can share Arc across threads.
  messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize)]
struct IndexResponse {
  server_id: usize,
  request_count: usize,
  messages: Vec<String>,
}

#[get("/")]
async fn index(req:HttpRequest) -> Result<web::Json<IndexResponse>> {

  let hello = req
      .headers()
      .get("hello")
      .and_then(|v| v.to_str().ok())
      .unwrap_or_else(|| "world");

  Ok(web::Json(IndexResponse {
      server_id: 22,
      request_count: 33,
      messages: vec![hello.to_owned()],
  }))
}

pub struct MessageApp {
  port: u16,
}

impl MessageApp {

  pub fn new(port: u16) -> Self {
    MessageApp { port }
  }

  pub async fn run(&self) -> std::io::Result<()> {

    let addr = format!("127.0.0.1:{}", self.port);
    println!("Starting http server:{}", addr);

    HttpServer::new(move || {

      App::new()
        // enable logger
        .wrap(middleware::Logger::default())
        .service(index)
    })
    .bind(addr)?
    .workers(8)
    .run()
    .await
  }
}
