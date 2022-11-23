# [Chapter 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

- Functions are prevalent in Rust code.
- One of the most important functions in the language: the `main` function, which is the entry point of many programs.
- `fn` keyword, which allows you to declare new functions.
- Snake case is the conventional style for function and variable names.
- Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
- In function signatures, you must declare the type of each parameter.

## [Parameters](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#parameters)

- Functions can have parameters.
- Concrete values passed to function are called arguments.

## [Statements and Expressions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions)

- Function bodies are made up of a series of statements optionally ending in an expression.
- Rust is an expression-based language.

### Statements

- Instructions that perform some action and do not return a value.
- e.g.
```rust
fn main() {
    let y = 6;
}
```
- Function definitions are also statements; the entire example above is a statement in itself.
- Statements do not return values.
  - You can’t assign a let statement to another variable
  ```rust
  // [This code does not compile!]
  fn main() {
      let x = (let y = 6); // error: expected expression, found statement (`let`)
  }
  ```

The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.

In those languages, you can write `x = y = 6` and have both x and y have the value 6; that is not the case in Rust.

### Expressions

- Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
- Math operation, such as 5 + 6, are expressions because they evaluates to a value.
- Expressions can be part of statements: e.g in the statement `let y = 6;` is an expression that evaluates to the value 6.
- Calling a function is an expression.
- Calling a macro is an expression.
- A new scope block created with curly brackets is an expression.
- Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

## [Functions with Return Values](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values)

- Functions can return values.
- Must declare their type after an arrow (->).
- Return value of the function is synonymous with the value of the final expression in the block of the body of a function.
- Return early from a function by using the `return` keyword and specifying a value

## [Basic Comments](https://doc.rust-lang.org/book/ch03-04-comments.html#comments)

- Double slash

