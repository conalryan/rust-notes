// The structopt crate defines a trait StructOpt and a custom derive which allows you to derive that trait for a type you create.
// These two pieces together create a system for declaratively specifying how your application takes input from the command line.
// Underlying structopt is another crate called clap. 
// You can, and many people do, build a command line application directly by interacting with the clap crate.
use structopt::StructOpt;
use heck::TitleCase;
use log::trace;

// As you should recall, this tells the compiler to look for files (or directories) with those names and to insert that code here with the appropriate scoping.
mod app;
mod client;
mod errors;

// We next use a use statement to bring our to be written error type into scope to make our type signatures easier to write:
use errors::HurlError;

// One feature we want to support is pretty printing JSON responses. 
// An aspect of pretty printing that can be quite handy is making sure that the keys are sorted. 
// JSON objects do not have a well-defined order so we are free to print them anyway we want. 
// There is an argument that we should faithfully represent the result based on the order of the bytes from the server.
// However, we are writing our own tool and are therefore free to make decisions such as this.
//
// Rust has multiple hash map implementations in the standard library, one in particular is the BTreeMap which stores entires sorted by the key.
type OrderedJson = std::collections::BTreeMap<String, serde_json::Value>;

fn main() -> HurlResult<()> {
    // Call from_args which is defined on the StructOpt trait. 
    // Therefore, we need a struct called App in the app module which implements this trait. 
    // This does all of the command line argument parsing including exiting if something can’t be parsed as well as printing a help message when necessary.
    let mut app = app::App::from_args();
    // We then have a call to validate which uses the ? operator to exit early if this function fails. 
    // This is not part of the argument parsing that StructOpt does. 
    // Rather, this is for handling certain constraints on the arguments that StructOpt is unable to enforce.
    app.validate()?;

    // Use log_level method on our app to get a value to setup logging. 
    // The pretty_env_logger crate uses environment variables to configure what to print, so we explicitly set the RUST_LOG environment variable based on the value we get. 
    // The format is RUST_LOG=binary_name=level where binary_name is not surprisingly the name of
    // your binary and level is one of the five level values that log defines: 
    // trace, debug, info, warn, and error.
    if let Some(level) = app.log_level() {
        std::env::set_var("RUST_LOG", format!("hurl={}", level));
        pretty_env_logger::init();
    }

    // The second piece is the heart of our application. 
    // We use the cmd (short for command), property on our app to direct what type of request to make. 
    // There are two cases:
    // 1. either we got a command which specifies the HTTP verb to use Some(ref method), 
    // in that case we use the client module to make the request and then call a handle_response function with the result.
    // 2. If we did not get a command, i.e. app.cmd matches None, then we are in the default case where we just got a URL. 
    // In this case, we make a GET request if we do not have any data arguments, otherwise we make a POST request. 
    // We also call a method on the client module to make this request and pipe through to the same handle_response function.
    match app.cmd {
        Some(ref method) => {
            let resp = client::perform_method(&app, method)?;
            handle_response(resp)
        }
        None => {
            let url = app.url.take().unwrap();
            let has_data = app.parameters.iter().any(|p| p.is_data());
            let method = if has_data {
                reqwest::Method::POST
            } else {
                reqwest::Method::GET
            };
            let resp = client::perform(&app, method, &url, &app.parameters)?;
            handle_response(resp)
        }
    }
}

// First, the signature. We expect a response as input, which in this case is just the
// Response type from the reqwest crate, and we return our result type.
fn handle_response(
    mut resp: reqwest::Response,
) -> HurlResult<()> {
    // Status
    // Get status and version of HTTP e.g. “HTTP/1.1 200 OK” or “HTTP/1.1 403 Forbidden”. 
    let status = resp.status();
    let mut s = format!(
        "{:?} {} {}\n",
        resp.version(),
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown")
    );
    
    // Headers
    // Gather headers into vector
    let mut headers = Vec::new();
    // The response type has a headers function which returns a reference to a Header type that gives us access to the headers. 
    // We have to explicitly turn it into an iterator by calling the iter so that we can process each key/value pair.
    for (key, value) in resp.headers().iter() {
        // The to_title_case method is available because we imported the TitleCase trait from heck.
        let nice_key = key.as_str().to_title_case().replace(' ', "-");
        headers.push(format!(
            "{}: {}",
            nice_key,
            value.to_str().unwrap_or("BAD HEADER VALUE")
        ));
    }
    // One special exception is content length. The reqwest crate does not treat content length as a normal header value 
    // and instead provides a content_length function on the response type to get this value. 
    // Let’s use this function to get a content length to add to your list of headers:
    // this function returns an Option and thus we have to deal with computing a value if this function returns None.
    // The content length of the response is not necessarily the same as the content length that is given in the response header 
    // because the actually response body could be compressed. 
    // After decompressing the body, we end up with a different length. 
    // The library returns None in this case to signal that if you want to compute an accurate content length, you have to do it yourself.
    let result = resp.text()?;
    let content_length = match resp.content_length() {
        Some(len) => len,
        None => result.len() as u64,
    };
    headers.push(format!("Content-Length: {}", content_length));
    // We put the headers into a vector so that we can sort by the name of the header here.
    headers.sort();
    // As join is not a method on Vec, we have to use &headers[..] to get a reference to a slice of type &[String]. 
    // We then turn the output of that function from String to &str with the extra & at the beginning. 
    // This allows us to pass the result to push_str which appends onto our already constructed status string s.
    s.push_str(&(&headers[..]).join("\n"));
    println!("{}", s);

    // Body
    // Pretty print Body
    let result_json: serde_json::Result<OrderedJson> = serde_json::from_str(&result);
    match result_json {
        Ok(result_value) => {
            let result_str = serde_json::to_string_pretty(&result_value)?;
            println!("{}", result_str);
        }
        Err(e) => {
            trace!("Failed to parse result to JSON: {}", e);
            println!("{}", result);
        }
    }

    Ok(())
}
