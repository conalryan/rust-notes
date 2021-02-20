use messages_actix::MessageApp;

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
fn main() -> std::io::Result<()> {
    // Hard code env var here for convenience.
    // so that we don’t have to set the environment variable in our terminal,
    // normally you would set this in your environment.
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Those logging statements do not actually do anything unless a program is configured with an implementation.
    // We choose to use the implementation provided by the env_- logger crate which we turn on with the call to env_logger::init().
    env_logger::init();
    let app = MessageApp::new(8080);
    app.run()
}
