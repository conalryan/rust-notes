# [Chapter 3.2: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

- Every value in Rust is of a certain data type.
- Two data type subsets: scalar and compound.
- Rust is a statically typed language, which means that it must know the types of all variables at compile time.
- Compiler can usually infer what type we want.

## [Scalar Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

- A scalar type represents a single value.
- Rust has four primary scalar types:
  1. integers
  2. floating-point numbers
  3. Booleans
  4. characters

### [1. Integers](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

- An integer is a number without a fractional component.
- Integer types default to i32

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

The isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

| Number literals | Example |
|---|---|
Decimal |	98_222
Hex |	0xff
Octal |	0o77
Binary | 0b1111_0000
Byte (u8 only) | b'A'

### [2. Floating-Point Numbers](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types)

- Two primitive types for floating-point numbers, which are numbers with decimal points
  1. f32 (32 bits)
  2. f64 (64 bits)
- Default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
- All floating-point types are signed.

```rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```

### [3. Boolan Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)

- Booleans are one byte in size.

```rust
let t = true;
let f: bool = false; // with explicit type annotation
```

### [4. Character Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

- Specify char literals with single quotes, as opposed to string literals, which use double quotes.
- four bytes in size and represents a Unicode Scalar Value.
- Unicode Scalar Value: represent more than just ASCII, Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces.
- Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

```rust
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

## [Compound Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

- Compound types can group multiple values into one type.
- Rust has two primitive compound types:
  1. tuples
  2. arrays

### [1. Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

- A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- We create a tuple by writing a comma-separated list of values inside parentheses.
- Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
- Destructuring: break tuple apart into variables
- We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
- Tuple without any values is called, `unit`.
  - This value and its corresponding type are both written () and represent an empty value or an empty return type.
  - Expressions implicitly return the unit value if they don’t return any other value.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;

let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

### [2. Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)

- Collection of multiple values is with an array.
- Unlike a tuple, every element of an array must have the same type.
- Arrays have a fixed length.
- Write the values in an array as a comma-separated list inside square brackets
- Arrays are useful when you want your data allocated on the stack rather than the heap

```rust
let a = [1, 2, 3, 4, 5];

let months = ["January", "February", "March", "April", "May", "June", "July",
          "August", "September", "October", "November", "December"];
```
