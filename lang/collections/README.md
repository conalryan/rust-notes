# [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html#common-collections)

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## [Vectors aka Vec<T>](https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors)

- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
- Vectors can only store values of the same type.
- They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.


## [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

Strings Are Not So Simple

To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

## [Challenges](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary)

Here are some exercises you should now be equipped to solve:
  - Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
  - Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
  - Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!