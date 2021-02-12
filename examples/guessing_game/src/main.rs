//  bring that type into scope explicitly with a use statement.
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // An associated function is implemented on a type, in this case String, rather than on a particular instance of a String.
        // Some languages call this a static method.
        let mut guess = String::new();

        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
        //
        // The next part of the code, .read_line(&mut guess), calls the read_line method on the standard input handle to get input from the user.
        // We’re also passing one argument to read_line: &mut guess.
        io::stdin()
            // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data
            // without needing to copy that data into memory multiple times.
            // Like variables, references are immutable by default.
            // Hence, you need to write &mut guess rather than &guess to make it mutable.
            .read_line(&mut guess)
            // The Result types are enumerations, often referred to as enums.
            // An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
            //
            // For Result, the variants are Ok or Err.
            // The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value.
            // The Err variant means the operation failed, and Err contains information about how or why the operation failed.
            //
            // An instance of io::Result has an expect method that you can call.
            // If this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // Shaddowing to convert type
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
