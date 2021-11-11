
// Structs
// Similar to tuples, each pieces of a struct can be different types.
// Unlike tuples, you’ll name each piece of data so it’s clear what the values mean.
// Structs are more flexible than tuples, the order of the data does not matter because they are named.
// Each peice of data is called a field in a struct.
//
// Add #[derive(Debug)] to struct to use {:?} syntax e.g. println!("user1 is {:?}", user1);
// Use {:#?} instead of {:?} to pretty print
#[derive(Debug)]
struct User {
    // Use the owned String type rather than the &str string slice type.
    // Intentional, we want instances of this struct to own all of its data
    // and for that data to be valid for as long as the entire struct is valid.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Encapsulate Functionality
// Put all the things we can do with an instance of a type in one impl block,
// rather than making future users of our code search for capabilities of our struct in various places in the library we provide.
//
// cr. so a class?
impl Rectangle {

    // Methods
    // Similar to functions:
    // - Declared with fn keyword and name of the function.
    // - Can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else.
    // Different than functions:
    // - Methods defined within the context of a struct or an enum or a trait object.
    // - First parameter is always self, which represents the instance of the struct the method is being called on.
    //
    // The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
    // & syntax because we don't want to take ownership, we just want to read, not write to it.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn build_user_verbose(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Field init shorthand syntax
// When the parameter names and the struct field names are exactly the same there is no need to respecify the field name (e.g. email: email).
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {

    // Create an instance of a struct
    // Specify concrete values for each of the fields.
    // We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs,
    // where the keys are the names of the fields and the values are the data we want to store in those fields.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("-- Structs n' Stuff! --");

    // Add #[derive(Debug)] to struct to use {:?} syntax
    println!("user1 is {:?}", user1);
    println!("user1 is {:#?}", user1);

    // dot notation to access field
    println!("user1.email: {}", user1.email);

    // mutate a value
    println!("-- Mutable: --");
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut mut_user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    mut_user1.email = String::from("anotheremail@example.com");
    mut_user1.username = String::from("anotherusername456");
    println!("mut_user1.email: {}; mut_user1.username: {}", mut_user1.email, mut_user1.username);

    // Struct Update Syntax
    println!("-- Struct Update Syntax: --");
    // Creating Instances From Other Instances.
    // Where most of an old instance’s values are kept but only a field fields are changed.
    //
    // cr. similar to js spread syntax
    let user2_struct_update_syntax = User {
        email: String::from("spread@example.com"),
        username: String::from("spread123"),
        ..user1
    };
    println!("user2_struct_update_syntax.email: {}; user2_struct_update_syntax.username: {}", user2_struct_update_syntax.email, user2_struct_update_syntax.username);

    // Tuple Structs
    println!("-- Tuple Structs: --");
    // You can also define structs that look similar to tuples, called tuple structs.
    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather,
    // they just have the types of the fields.
    //
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples,
    // and naming each field as in a regular struct would be verbose or redundant.
    //
    // Function parameter of a named tuple struct will not accept another tuple sturct even if the values are the same.
    // Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces,
    // you can use a . followed by the index to access an individual value e.g. Color.0, Color.1
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(4, 22, 60);
    println!("tuple struct: {}", origin.1);

    println!("-- Methods: --");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
