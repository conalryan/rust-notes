use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("\n Error Handling");

    // Unrecoverable Errors with panic!
    // --------------------------------
    // There are two ways to cause a panic in practice:
    // 1. By taking an action that causes our code to panic (such as accessing an array past the end).
    // 2. By explicitly calling the panic! macro. In both cases, we cause a panic in our program.
    //
    // By default, these panics will print a failure message, unwind, clean up the stack, and quit.
    // Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.
    //
    // If you want to the program to end immediately without cleanup
    // If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort'
    // to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
    // [profile.release]
    // panic = 'abort'

    // Call panic!
    println!("call panic!() directiley");
    // panic!("crash and burn"); // thread 'main' panicked at 'crash and burn', src/main.rs:18:5

    // Automatic panic due to index out of bounds
    let v = vec![1, 2, 3];
    // v[99]; // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:22:5

    // In C, attempting to read beyond the end of a data structure is undefined behavior.
    // You might get whatever is at the location in memory that would correspond to that element in the data structure,
    // even though the memory doesn’t belong to that structure. This is called a buffer overread and can lead to security vulnerabilities
    // if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.

    // Recoverable Errors with Result
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let greeting_file_result = File::open("does_not_exist.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Matching on Different Errors
    // The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation.
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Closure insted of match with Result<T, E>
    // ----------------------------------------------

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    // ------------------------------------------------
    // The unwrap method is a shortcut method implemented just like a match expression.
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic!
    let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("does_not_exist.txt").unwrap();
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:83:58

    // the expect method lets us also choose the panic! error message.
    // Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
    // In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed.
    let greeting_file = File::open("does_not_exist.txt")
        ;// .expect("does_not_exist.txt should be included in this project");
        // thread 'main' panicked at 'does_not_exist.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:88:10

    // Propagating Errors
    // ------------------
    // When a function’s implementation calls something that might fail, instead of handling the error within the function itself,
    // you can return the error to the calling code so that it can decide what to do.

    // Example function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.

    // The return type of the function: Result<String, io::Error> means the function is returning a value of the type Result<T, E>
    // where the generic parameter T has been filled in with the concrete type String,
    // and the generic type E has been filled in with the concrete type io::Error.
    //
    // If function succeeds, the client will receive an Ok value that holds a String of the contents of the file.
    // If function fails, the client will receive an Err value that holds an instance of io::Error with detais of the error.
    fn read_from_file() -> Result<String, io::Error> {
        let from_file = File::open("hello.txt");

        let mut from_file_result = match from_file {
            Ok(file) => file,
            // Instead of calling panic!, we use the return keyword to return early out of the function entirely
            // and pass the error value from File::open, now in the pattern variable e,
            // back to the calling code as this function’s error value.
            Err(e) => return Err(e),
        };

        let mut from_file_content = String::new();

        // The read_to_string method also returns a Result because it might fail, even though File::open succeeded.
        // So we need another match to handle that Result: if read_to_string succeeds, then our function has succeeded,
        // and we return the username from the file that’s now in username wrapped in an Ok.
        // If read_to_string fails, we return the error value in the same way that we returned the error value in the match that handled the return value of File::open.
        // However, we don’t need to explicitly say return, because this is the last expression in the function.
        match from_file_result.read_to_string(&mut from_file_content) {
            Ok(_) => Ok(from_file_content),
            Err(e) => Err(e),
        }
    }

    // It’s up to the calling code to decide what to do with those values.
    // If the calling code gets an Err value, it could call panic! and crash the program, use a default username,
    // or look up the username from somewhere other than a file, for example.
    //
    // We don’t have enough information on what the calling code is actually trying to do,
    // so we propagate all the success or error information upward for it to handle appropriately.
    let read_from_file_result = read_from_file();

    match read_from_file_result {
        Ok(r) => println!("read_from_file_result SUCCESS: {r}"),
        Err(e) => println!("read_from_file_result ERROR: {e}"),
    }

    // A Shortcut for Propagating Errors: the ? Operator
    // -------------------------------------------------

    fn read_from_file_shortcut() -> Result<String, io::Error> {
        // The ? placed after a Result value will work similar to match expressions.
        // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression.
        // If the value is an Err, the Err will be returned from the whole function.
        let mut from_file_result = File::open("hello.txt")?;
        let mut from_file = String::new();
        // There is a difference between what the match expression does and what the ? operator does:
        // error values that have the ? operator called on them go through the from function,
        // defined in the From trait in the standard library, which is used to convert values from one type into another.
        // When the ? operator calls the from function, the error type received is converted into the error type defined
        // in the return type of the current function. This is useful when a function returns one error type to represent
        // all the ways a function might fail, even if parts might fail for many different reasons.
        //
        // If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code.
        from_file_result.read_to_string(&mut from_file)?;
        Ok(from_file)
    }

    // Simplified
    fn read_from_file_shortcut_chain() -> Result<String, io::Error> {
        let mut contents = String::new();
        File::open("hello.txt")?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    // Ultra-simplified
    fn read_from_file_ultra_simplified() -> Result<String, io::Error> {
        // Reading a file into a string is a fairly common operation, so the standard library provides the convenient
        // fs::read_to_string function that opens the file, creates a new String, reads the contents of the file,
        // puts the contents into that String, and returns it.
        fs::read_to_string("hello.txt")
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        // Note that you can use the ? operator on a Result in a function that returns Result,
        // and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.
        // The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases,
        // you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.
        text.lines().next()?.chars().last()
    }
}
