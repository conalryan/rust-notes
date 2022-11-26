// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
pub fn run() {

  println!("\nHashMaps");

  // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function,
  // which determines how it places these keys and values into memory.
  // Many programming languages support this kind of data structure, but they often use a different name,
  // such as hash, map, object, hash table, dictionary, or associative array, just to name a few.
  //
  // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.
  // This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
  // You can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait.
  use std::collections::HashMap;

  // Create a HashMap
  // ----------------
  // Just like vectors, hash maps store their data on the heap.
  // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other,
  // and all of the values must have the same type.
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // Accessing Values using get method
  // ---------------------------------
  // get method returns an Option<&V>
  let team_name = String::from("Blue");
  // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>,
  // then unwrap_or to set score to zero if scores doesn't have an entry for the key.
  let score = scores.get(&team_name).copied().unwrap_or(0);

  // Iterate over key/values using for loop
  // --------------------------------------
  // We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:
  for (key, value) in &scores {
      println!("{}: {}", key, value);
  }

  // Ownership
  // ---------
  // For types that implement the Copy trait, like i32, the values are copied into the hash map.
  // For owned values like String, the values will be moved and the hash map will be the owner of those values
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // field_name and field_value are invalid at this point, try using them and
  // see what compiler error you get!
  // println!("field_name is: {field_name}"); // error[E0382]: borrow of moved value: `field_name`

  // If we insert references to values into the hash map, the values won’t be moved into the hash map.
  // The values that the references point to must be valid for at least as long as the hash map is valid.

  // Updating a HashMap

  // Overwriting a Value
  // ------------------
  // If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  println!("{:?}", scores);

  // Adding a Key and Value Only If a Key Isn’t Present
  // --------------------------------------------------
  // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
  // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
  scores.entry(String::from("Yellow")).or_insert(200);
  scores.entry(String::from("Blue")).or_insert(50);
  scores.entry(String::from("Green")).or_insert(1);
  println!("{:?}", scores);

  // Updating a Value Based on the Old Value
  // ---------------------------------------
  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      // in order to assign to that value, we must first dereference count using the asterisk (*)
      *count += 1;
  }

  println!("{:?}", map);
}