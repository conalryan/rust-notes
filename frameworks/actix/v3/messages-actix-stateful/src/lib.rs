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
//
// 1. Cell
// Cell implements interior mutability by moving values in and out of a shared memory location.
//
// 2. RefCell (cr. why use this, over cell?)
// RefCell implements interior mutability by using borrow checking at runtime to enforce the constraint
// that only one mutable reference can be live at any given time.
// If one tries to mutably borrow a RefCell that is already mutably borrowed the calling thread will panic.
// cr. having a solution that results in runtime errors seem very brittle and limited.
//
// Cell and RefCell are not needed that often in everyday Rust
struct AppState {
  server_id: usize,
  request_count: Cell<usize>,
  // We can ensure mutually exclusive access to the vector by creating a Mutex that wraps our vector.
  //
  // Typically each value in Rust has a single owner, but for this situation we want each thread to be an owner of the data
  // so that the vector lives until the last worker thread exits.
  // The mechanism for this in Rust is to use a reference counted pointer.
  // There are two variants:
  //
  // 1. Rc<T> (not thread safe)...bottom line is don't use this! @see 2. Arc below.
  // Calling clone on an Rc will produce a new pointer to the same value on the heap.
  // When the last Rc pointer to a value is destroyed, the pointed-to value will then be destroyed.
  // Rc is non-atomic and therefore not thread safe.
  // You cannot share Rc across threads.
  // Uses a trait called Deref to alow you to call the methods of T directily on a value of type Rc<T>
  // As Rust does not have a garbage collector, it is possible to create memory leaks by creating cycles of reference counted pointers.
  // There is a non-owning variant called Weak which can be used to break such cycles.
  // This is not an issue for us here, but it is important to be aware of especially if you are coming from a garbage collected language.
  //
  // 2. Arc (thread safe)
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

// Extractors
// It may seem a bit magical to just define the input parameter of our handler to be the state rather than having to figure out how to get that from our server or the request.
// The mechanism that allows this is a trait called FromRequest and the generic term for this concept is an extractor.
// Extractors are types that implement the FromRequest trait which allow types to define how they are constructed from a request.
//
// FromRequest trait
// Any type that implements FromRequest can technically fail to extract said type and thus uses Result in the implementation.
// You can define your handler to take a Result<T> or an Option<T> for any T that implements FromRequest to be able to handle the failure of extraction in your handler.
#[get("/")]
async fn index(state:web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

  let request_count = state.request_count.get() + 1;
  // The reason that we cannot mutate request_count directly is that our state variable is an immutable reference.
  // There is no way for us to update server_id for example, therefore we use set and get
  state.request_count.set(request_count);
  // To get access to the data inside the mutex we call the lock method on the mutex.
  // The lock method blocks until the underlying operating system mutex is not held by another thread.
  // This method returns a Result wrapped around a MutexGuard which is wrapped around our data.
  // The Result that is returned will be an error only if the mutex is poisoned which basically means
  // a thread paniced while holding the lock and likely your program is in a bad state.
  //
  // Often you will see lock().unwrap() used with mutexes.
  // The type of the variable we get from state.messages.lock().unwrap() is actually a MutexGuard<Vec<String>>
  //
  // RAII (Resource Acquisitions Is Initialization)
  // Pattern for managing resources which is central to Rust. In particular, when a value goes out of scope,
  // a special method called drop is called by the compiler if the type of the value implements the Drop trait.
  // For a MutexGuard, the mutex is locked when the guard is constructed and the lock is unlocked in the guard’s drop method.
  //
  // Therefore, the lock is only locked for as long as you have access to the guard.
  // Additionally, you only have access to the data protected by the mutex through the guard.
  // Hence, the data is only accessible while the lock is locked.
  // You don’t have to worry about calling unlock at the right time or ensuring that you actually locked the mutex in all the places
  // that you read or write the vector of messages. All of that is taken care of for you by the compiler.
  let ms = state.messages.lock().unwrap();

  Ok(web::Json(IndexResponse {
      server_id: state.server_id,
      request_count,
      // The clone method creates an explicit copy of a value if the type implements the Clone trait.
      // We cannot just pass the messages vector directly because that would move ownership and that is not what we want to do (nor even possible because it is shared).
      // We want to return a copy of the vector of messages to be serialized.
      // Because this copying might be expensive Rust does not do it implicitly,
      // rather you are required to state that you want it to happen explicitly by calling clone.
      //
      // Copy trait
      // For things that can be copied cheaply, there is a separate trait called Copy which will result in implicit copies being created.
      messages: ms.clone(),
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

    // Create the shared messages vector outside of the application factory closure.
    // We do this so that each worker can actually share the same messages array
    // rather than each of them creating their own vector which would be unconnected from the other workers.
    let messages = Arc::new(Mutex::new(vec!["foo".to_owned()]));

    HttpServer::new(move || {

      App::new()
      .data(AppState {
        // The second argument to fetch_add controls how atomic operations synchronize memory across threads.
        // The strongest ordering is SeqCst which stands for sequentially consistent.
        // The best advice is to use SeqCst until you profile your code, find out that this is a hot spot,
        // and then can prove that you are able to use one of the weaker orderings based on your access pattern.
        server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
        request_count: Cell::new(0),
        // We have to clone the message as we push it into the vector because this vector owns each element and we only have a borrowed reference to our PostInput data.
        messages: messages.clone(),
        })
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
