fn main() {

    // -----------------------------------------------------------------------
    println!("[IF ELSE]");

    let number = 6;
    // Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // -----------------------------------------------------------------------
    println!("[IF LET STATEMENT]");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // -----------------------------------------------------------------------
    println!("[LOOPS]");

    // Uncomment to blow up
    // loop {
    //     println!("again!");
    // }

    // Return value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // -----------------------------------------------------------------------
    println!("[WHILE LOOP]");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // -----------------------------------------------------------------------
    println!("[FOR LOOP]");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // -----------------------------------------------------------------------
    println!("[RANGE]");

    // Range, provided by the standard library, which generates all numbers in sequence.
    // Example below uses range plus rev() to reverse the order.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
