// Vectors https://doc.rust-lang.org/book/ch08-01-vectors.html
pub fn run() {
  println!("\nVectors:");

  // Note that we added a type annotation here.
  // Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
  // Vectors are implemented using generics.
  let v: Vec<i32> = Vec::new();

  // Create using the vec macro
  // Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary.
  // You must declare the vector as mut in order to modify it. This seem incredibly ironic, considering you would just use
  // an array if you never intended to modify it. The entire use case around a vector is that it can grow in size!
  let mut v = vec![1, 2, 3];

  // Use push to add values to vector.
  // As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword.
  //
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  // Reading Elements of Vectors
  // There are two ways to reference a value stored in a vector: via indexing or using the get method.
  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  let third: Option<&i32> = v.get(2);
  match third {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
  }

  // let does_not_exist = &v[100]; // thread 'main' panicked at 'index out of bounds: the len is 7 but the index is 100', src/main.rs:36:27
  // same behavior has python get in a dictionary ref:https://github.com/conalryan/python-notes/blob/master/data_types/mapping_types.py#L39
  // When the get method is passed an index that is outside the vector, it returns None without panicking.
  let does_not_exist = v.get(100);
  match does_not_exist {
      Some(not_gonna_happen) => println!("this will never print"),
      None => println!("Duh"),
  }

  // When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules of so that the vector remain valid.
  let first = &v[0];
  v.push(6);

  // This error is due to the way vectors work: because vectors put the values next to each other in memory,
  // adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
  // if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
  // In that case, the reference to the first element would be pointing to deallocated memory.
  // The borrowing rules prevent programs from ending up in that situation.
  // println!("The first element is: {}", first); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

  // Iterate over vector
  for i in &v {
      println!("{}", i);
  }

  // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
  // The for loop in Listing 8-8 will add 50 to each element.
  for i in &mut v {
      // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
      *i += 50;
  }

  // Using an Enum to Store Multiple Types
  // Vectors can only store values that are the same type.
  // This can be inconvenient; there are definitely use cases for needing to store a list of items of different types.
  //
  // Variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types,
  // we can define and use an enum!
  //
  // For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers,
  // some floating-point numbers, and some strings.
  // We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type:
  // that of the enum. Then we can create a vector to hold that enum and so, ultimately, holds different types
  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];

  // Like any other struct, a vector is freed when it goes out of scope.
  // When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.
  // The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
  {
      let v = vec![1, 2, 3, 4];

      // do stuff with v
  } // <- v goes out of scope and is freed here
}