# [Chapter 3.5: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

- Ability to run code depending on a condition.
- Most common constructs that let you control the flow of execution are:
  1. if expressions
  2. loops

## [1. if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

- An if expression allows you to branch your code depending on conditions.
- Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions
- Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.

### [Using  if in a let statement](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement)

- Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable

## [2. Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)

- Rust has three kinds of loops:
  1. loop
  2. while
  3. for

### [1. Loop](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repeating-code-with-loop)

- You can place the `break` keyword within the loop to tell the program when to stop executing the loop.
- `continue` keyword tells a loop to skip over any remaining code in this iteration of the loop and go to the next iteration.

#### [Returning Values from Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops)

- Example usecase of loop is to retry an operation you know might fail, such as checking whether a thread has completed its job.
- You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it.

#### [Loop Labels to Disambiguate Between Multiple Lines](https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops)

If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

#### [Conditioal Loops with While](https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while)

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

#### [Looping Through a Collection with for](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```