use automated_tests;

// Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
// To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file:
// $ cargo test --test integration_tests
//
// No Integration Tests for Binary Crates
// If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests.
// Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.
//
// This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
// Using that structure, integration tests can test the library crate with use to make the important functionality available.
// If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
#[test]
fn it_adds_two() {
    assert_eq!(4, automated_tests::add_two(2));
}