
fn main() {

    // --- Floating-Point Numbers --- //

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // --- Numeric Operations --- //

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;


    // --- Boolean --- //

    let t = true;

    let f: bool = false; // with explicit type annotation

    // --- Char --- //

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // --- Tuple --- //

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // Get values using dot syntax
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // --- Array --- //

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let a = [3; 5];

    // access elements of an array using indexing
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];


}
