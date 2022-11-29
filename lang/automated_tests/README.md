# [Chapter 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

## [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

The Rust community thinks about tests in terms of two main categories: unit tests and integration tests.
Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
Integration tests are entirely external to your library and use your code in the same way any other external code would,
using only the public interface and potentially exercising multiple modules per test.

### [Unit Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests)

The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
You’ll put unit tests in the src directory in each file with the code that they’re testing.
The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).

### [The Tests Module and #[cfg(test)]](https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-module-and-cfgtest)

The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.

### Testing Private Functions
There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions. Regardless of which testing ideology you adhere to,
Rust’s privacy rules do allow you to test private functions.

### [Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)

In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well.

The tests Directory

We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.