# [I/O Project: Building Commmand Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

## [Accetping Command Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#accepting-command-line-arguments)

The first task is to make minigrep accept its two command line arguments: the file path and a string to search for. That is, we want to be able to run our program with cargo run, two hyphens to indicate the following arguments are for our program rather than for cargo, a string to search for, and a path to a file to search in, like so:

`cargo run -- searchstring example-filename.txt`

We'll need `std::env::args` function provided in Rust’s standard library.
This function returns an iterator of the command line arguments passed to the function.

## [Reading a File](https://doc.rust-lang.org/stable/book/ch12-02-reading-a-file.html#reading-a-file)

The `main` function has multiple responsibilities: generally, functions are clearer and easier to maintain if each function is responsible for only one idea.
The other problem is that we’re not handling errors as well as we could.
The program is still small, so these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly.
It’s good practice to begin refactoring early on when developing a program, because it’s much easier to refactor smaller amounts of code.

## [Refactoring to Improve Modularity and Error Handling](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#refactoring-to-improve-modularity-and-error-handling)

First, our main function now performs two tasks: it parses arguments and reads files.
As our program grows, the number of separate tasks the main function handles will increase.
As a function gains responsibilities, it becomes more difficult to reason about, harder to test, and harder to change without breaking one of its parts.
It’s best to separate functionality so each function is responsible for one task.

### [Separation of Concerns](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects)

- Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand.

## [Challenge]
For another exercise on your own, try controlling case sensitivity through either a command line argument or an environment variable. Decide whether the command line argument or the environment variable should take precedence if the program is run with one set to case sensitive and one set to ignore case.

## [Writing Error Messages to Standard Error Instead of Standard Output](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html)

At the moment, we’re writing all of our output to the terminal using the println! macro. In most terminals, there are two kinds of output: standard output (stdout) for general information and standard error (stderr) for error messages.

Redirect the standard output stream to a file
The > syntax tells the shell to write the contents of standard output to output.txt instead of the screen.
`cargo run > output.txt`

