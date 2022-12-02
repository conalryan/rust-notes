// First, we bring the std::env module into scope with a use statement so we can use its args function.
// Notice that the std::env::args function is nested in two levels of modules, but we only pull in
// the parent module so the namespaces are clear.
use std::env;
use std::process;
use minigrep::Config;

// cargo run -- needle haystack
// cargo run -> &args = ["target/debug/minigrep",],
// cargo run -- needle haystack -> &args = ["target/debug/minigrep","needle","haystack",]
// cargo run -- frog poem.txt
// cargo run -- body poem.txt
// cargo run -- monomorphization poem.txt
// cargo run -- to poem.txt
// IGNORE_CASE=1 cargo run -- to poem.txt
// cargo run -- to poem.txt > output.txt
fn main() {
    // call env::args, and use collect to turn the iterator into a vector.
    // We can use the collect function to create many kinds of collections,
    // so we explicitly annotate the type of args to specify that we want a vector of strings.
    // Although we very rarely need to annotate types in Rust, collect is one function
    // you do often need to annotate because Rust isn’t able to infer the kind of collection you want.
    let args: Vec<String> = env::args().collect();
    // cr. originally I copied the example dbg!(args);, but of course down the line I get a move error.
    // Why not just always default to passing a refence to debug?
    // Why would you ever want to give ownership to debug?
    // dbg!(&args);

    // unwrap_or_else, which is defined on Result<T, E> by the standard library.
    // Using unwrap_or_else allows us to define some custom, non-panic! error handling.
    // If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping.
    // However, if the value is an Err value, this method calls the code in the closure,
    // which is an anonymous function we define and pass as an argument to unwrap_or_else.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        // The standard library provides the eprintln! macro that prints to the standard error stream
        // he process::exit function will stop the program immediately and return the number that was passed as the exit status code.
        // This is similar to the panic!-based handling we used, but we no longer get all the extra output (e.g. stack trace).
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does.
    // The run function doesn’t return a value that we want to unwrap in the same way that Config::build returns the Config instance.
    // Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value,
    // which would only be ().
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}