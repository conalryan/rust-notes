# [Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

To define a struct, we enter the keyword struct and name the entire struct.

### [Where's the -> Operator](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator)

In C and C++, two different operators are used for calling methods: you use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, `object->something()` is similar to `(*object).something()`.

Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```
The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
