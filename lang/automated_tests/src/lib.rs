use rand::Rng;

pub fn add_two(a: i32) -> i32 {
  a + 2
}

// For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types.
// You’ll also need to implement Debug to print the values when the assertion fails
#[derive(PartialEq, Debug)]
pub struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
   fn can_hold(&self, other: &Rectangle) -> bool {
       self.width > other.width && self.height > other.height
   }

   fn random_width(&self) -> u32 {
      let secret_number = rand::thread_rng().gen_range(1..101);
      self.width * secret_number
   }
}

// Rust can check the validaty of inputs and outputs but it cannot determine if the function is behaving correctly.
// That is where automated tests come in.
// We can write tests that assert, for example, that when we pass 3 to the add_two function, the returned value is 5.
// We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.
//
// The bodies of test functions typically perform these three actions:
// 1. Set up any needed data or state.
// 2. Run the code you want to test.
// 3. Assert the results are what you expect.
// cr. 4. (Optional) tear down after test.
//
// The Anatomy of a Test Function
// At its simplest, a test in Rust is a function that’s annotated with the test attribute.
// Attributes are metadata about pieces of Rust code; one example is the derive attribute
//
// We might also have non-test functions in the tests module to help set up common scenarios or perform common operations,
// so we always need to indicate which functions are tests.
#[cfg(test)]
mod tests {
  // Note that we’ve added a new line inside the tests module: use super::*;.
  // The tests module is a regular module that follows the usual visibility rules
  // Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
  // We use a glob here so anything we define in the outer module is available to this tests module.
  use super::*;

  #[test]
  fn it_works() {
      let result = 2 + 2;
      assert_eq!(result, 4);
  }

  #[test]
  fn exploration() {
      assert_eq!(2 + 2, 4);
  }

  #[test]
  fn it_adds_two() {
      // in Rust, they’re called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter.
      // We could write the assertion in this test as assert_eq!(add_two(2), 4), which would result in the same failure message that displays assertion failed: `(left == right)`.
      assert_eq!(4, add_two(2));
  }

  // We give the assert! macro an argument that evaluates to a Boolean. If the value is true, nothing happens and the test passes.
  // If the value is false, the assert! macro calls panic! to cause the test to fail. Using the assert!
  // #[test]
  // fn another() {
  //     panic!("Make this test fail");
  // }

  #[test]
  fn larger_can_hold_smaller() {
      let larger = Rectangle {
          width: 8,
          height: 7,
      };
      let smaller = Rectangle {
          width: 5,
          height: 1,
      };
      assert!(larger.can_hold(&smaller));
  }

  // Testing Equality with the assert_eq! and assert_ne! Macros
  // The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal.
  // This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely shouldn’t be.
  // The best thing to assert might be that the output of the function is not equal to the input.
  // Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively.
  // The values being compared must implement the PartialEq and Debug traits.
  #[test]
  fn test_ne() {
      let rect = Rectangle {
          width: 7,
          height: 5,
      };
      assert_ne!(rect.random_width(), rect.width);
  }
}

// Checking for Panics with should_panic

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
      if value < 1 || value > 100 {
          panic!("Guess value must be between 1 and 100, got {}.", value);
      }

      Guess { value }
  }
}

#[cfg(test)]
mod tests2 {
  use super::*;

  // We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to.
  //
  // To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute.
  // #[should_panic(expected = "less than or equal to 100")]
  // The failure message indicates that this test did indeed panic as we expected, but the panic message did not include the expected string 'Guess value must be less than or equal to 100'.
  #[test]
  // #[should_panic]
  #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
  fn greater_than_100() {
      Guess::new(200);
  }
}

// Using Result<T, E> in Tests
// Our tests so far all panic when they fail. We can also write tests that use Result<T, E>
#[cfg(test)]
mod tests3 {

  // The it_works function now has the Result<(), String> return type.
  // In the body of the function, rather than calling the assert_eq! macro,
  // we return Ok(()) when the test passes
  // and an Err with a String inside when the test fails.
  //
  // Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests,
  // which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
  //
  // You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant,
  // don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
  #[test]
  fn it_works() -> Result<(), String> {
      if 2 + 2 == 4 {
          Ok(())
      } else {
          Err(String::from("two plus two does not equal four"))
      }
  }
}


fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

// By default, if a test passes, Rust’s test library captures anything printed to standard output.
// For example, if we call println! in a test and the test passes, we won’t see the println! output in the terminal;
// we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.
//
// If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with --show-output.
// $ cargo test -- --show-output
#[cfg(test)]
mod tests4 {
  use super::*;

  #[test]
  fn this_test_will_pass() {
      let value = prints_and_returns_10(4);
      assert_eq!(10, value);
  }

  // #[test]
  // fn this_test_will_fail() {
  //     let value = prints_and_returns_10(8);
  //     assert_eq!(5, value);
  // }
}

// Running a Subset of Tests by Name
// Sometimes, running a full test suite can take a long time. If you’re working on code in a particular area,
// you might want to run only the tests pertaining to that code. You can choose which tests to run
// by passing cargo test the name or names of the test(s) you want to run as an argument.
//
// Running Single Tests
// We can pass the name of any test function to cargo test to run only that test:
// $ cargo test one_hundred
//
// Filtering to Run Multiple Tests
// We can specify part of a test name, and any test whose name matches that value will be run.
// $ cargo test add
//
// Also note that the module in which a test appears becomes part of the test’s name,
// so we can run all the tests in a module by filtering on the module’s name.
//
// This code is the automatically generated test module. The attribute cfg stands for configuration
// and tells Rust that the following item should only be included given a certain configuration option.
// In this case, the configuration option is test, which is provided by Rust for compiling and running tests.
#[cfg(test)]
mod tests5 {
  use super::*;

  #[test]
  fn add_two_and_two() {
      assert_eq!(4, add_two(2));
  }

  #[test]
  fn add_three_and_two() {
      assert_eq!(5, add_two(3));
  }

  #[test]
  fn one_hundred() {
      assert_eq!(102, add_two(100));
  }

  // Ignoring Some Tests Unless Specifically Requested
  // you can instead annotate the time-consuming tests using the ignore attribute to exclude them, as shown here:
  // The expensive_test function is listed as ignored. If we want to run only the ignored tests, we can use cargo test -- --ignored:
  // $ cargo test -- --ignored
  #[test]
  #[ignore]
  fn expensive_test() {
      // code that takes an hour to run
  }
}