fn main() {
    another_function();
    another_fn_with_param(7);
    let y  = expression_evaluate_to_value();
    println!("The value of y is: {y}");
}

// Declared after it was called. Rust does't care as long as it is in scope.
fn another_function() {
    println!("Another function.");
}

// In function signatures, you must declare the type of each parameter.
fn another_fn_with_param(num: i32) {
  println!("Another function with argument: {num}");
}

fn expression_evaluate_to_value() -> i32 {
    // Expressions evaluate to a value
    let y = {
        let x = 3;
        x + 1
    };
    // Return value
    y
}

// When defining multiple parameters, separate the parameter declarations with commas, like this:
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions with return value
fn fucntion_return_value() -> i32 {
    5
}