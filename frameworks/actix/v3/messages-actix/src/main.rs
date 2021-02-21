use actix_web::{get, web, App, HttpServer, Responder};

const PORT: u16 = 8080;

// Aggregate data types
// Primary mechanism for aggregate data types are:
// - enums
// - structs
//
// In theoretical terms, these are aggregates of other types where structs represent product types and enums represent sum types.
//
// Enums
// Enums in Rust can contain data and are therefore related to algebraic data types in functional languages.
// Enums represent a type where we enumerate the different possible values that a particular value represents.
// These are sometimes called tagged unions.
// use Color::Red or Color::Green
enum Color {
    Red,
    Green,
    Blue,
}

// http://localhost:8080/22/bob/index.html
#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

// Result is one of the primary error handling primitives that Rust provides from standard library.
// enum Result<T, E> {
//   Ok(T),
//   Err(E),
// }
// value of type Result can be in exactly one of two states: the Ok variant or the Err variant.
// Therefore, Result::Ok(true) would construct the Ok variant of the type Result<bool, E> where E would need to be further specified by the context.
// We will see Result used throughout all of the code we write in Rust because it allows us to handle success and failure cases in a structured and cohesive way.
// This is one of the tools that replace situations where one might use exceptions in another language.
// Note empty tuple response () is a placeholder to signify success, if there is an error, that
// will be returned otherwise empty tuple.
//
// std::io::Result<()> is the same as Result<(), std::io::Error>.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting http server: 127.0.0.1:{}", PORT);

    // Hard code env var here for convenience.
    // so that we don’t have to set the environment variable in our terminal,
    // normally you would set this in your environment.
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Those logging statements do not actually do anything unless a program is configured with an implementation.
    // We choose to use the implementation provided by the env_logger crate which we turn on with the call to env_logger::init().
    env_logger::init();

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
    // Move signifies intent that the function should not have references to the environment in
    // which it was created.
    HttpServer::new(||
        App::new()
        .service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await

    // HttpServer::new(move || {
    //     App::new()
    //         .wrap(middleware::Logger::default())
    //         .service(index)
    // })
    // // ? operator
    // // Common pattern of returning an error early if one occurred or otherwise pulling the value out of the Ok case and continuing on.
    // // Alternative syntax without ? operator
    // // let result = HttpServer::new(move || {
    // //  ...
    // // }).bind(("127.0.0.1", self.port));
    // // if result.is_err() {
    // //      return Err(result.err().unwrap());
    // // }
    // // result.unwrap().workers(8).run()
    // .bind(("127.0.0.1", self.port))?
    // .workers(8)
    // .run()

}
