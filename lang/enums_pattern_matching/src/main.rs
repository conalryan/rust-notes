// enums give you a way of saying a value is one of a possible set of values.
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
// This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
enum IpAddr {
    V4(String),
    V6(String),
}

// This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum!
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:
impl Message {
    fn call(&self) {
        // method body would be defined here
        dbg!(&self);
    }
}

// Or written as structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
// This is how we can extract values out of enum variants.
// Combining match and enums is useful in many situations.
// You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska).
// Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! Matches in Rust are exhaustive:
// we must exhaust every last possibility in order for the code to be valid.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // hat is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
    // We automatically get this constructor function defined as a result of defining the enum.
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option Enum
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-all Patterns and the _ Placeholder
    // Using enums, we can also take special actions for a few particular values, but for all other values take one default action.
    // Note that we have to put the catch-all arm last because the patterns are evaluated in order.
    // If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern:
    // _ is a special pattern that matches any value and does not bind to that value.
    // This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // Finally, we’ll change the rules of the game one more time, so that nothing else happens on your turn if you roll anything other than a 3 or a 7.
    // We can express that by using the unit value (the empty tuple type
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Concise Control Flow with if let
    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Using if let means less typing, less indentation, and less boilerplate code.
    //  However, you lose the exhaustive checking that match enforces.
    // Choosing between match and if let depends on what you’re doing in your particular situation
    // and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Match with catch-all or if let else
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

