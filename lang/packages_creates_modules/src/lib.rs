

mod back_of_house {
  // If we make an enum public, all of its variants are then public.
  // We only need the pub before the enum keyword.
  // This is a convenience since there is no reason to have an enum private,
  // the idea is to select one.
  pub enum Appetizer {
      Soup,
      Salad,
  }

  // if we use pub before a struct definition, we make the struct public,
  // but the struct’s fields will still be private.
  // We can make each field public or not on a case-by-case basis.
  pub struct Breakfast {
      // public
      pub toast: String,
      // private
      seasonal_fruit: String,
  }

  impl Breakfast {
      // private by default so need to use pub keyword to make it public.
      pub fn summer(toast: &str) -> Breakfast {
          Breakfast {
              toast: String::from(toast),
              seasonal_fruit: String::from("peaches"),
          }
      }
  }
}

mod front_of_house;

// create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.
// instead of crate::front_of_house::hosting::add_to_waitlist()
// we can use hosting::add_to_waitlist()
// Note that use only creates the shortcut for the particular scope in which the use occurs.
// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
// you could pull the function into scope with use crate::front_of_house::hosting::add_to_waitlist; then call it add_to_waitlist()
// but this makes it harder to determine if this is an external function or a local function.
// it’s idiomatic to specify the full path.
//
// pub use is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
// before change, client would need to call restaurant::front_of_house::hosting::add_to_waitlist()
// after pub use, client can call restaurant::hosting::add_to_waitlist()
// With pub use, we can write our code with one structure but expose a different structure.
pub use crate::front_of_house::hosting;

mod customer {
  pub fn eat_at_restaurant() {
      // This is not in scope, it will error: use of undeclared crate or module `hosting`
      // hosting::add_to_waitlist();
  }
}

// private by default so need to use pub keyword to make it public.
pub fn eat_at_restaurant() {
   // Absolute path
   crate::front_of_house::hosting::add_to_waitlist();

   // Relative path
   front_of_house::hosting::add_to_waitlist();

   // with use keyword
   hosting::add_to_waitlist();

  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

// Idiomatic use, pull in parent module then call
// function, struct, enum, etc.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
  Ok(())
}

// As syntax: as keyword and a new local name, or alias, for the type.
use std::io::Result as IoResult;

fn function3() -> IoResult<()> {
  Ok(())
}