
Fullstack Rust by Andrew Weiss
================================================================================

Introduction
--------------------------------------------------------------------------------
We are currently in the fifth era of programming language evolution.
This is an era where languages have been able to take all of the learnings since the 1950s and 
incorporate the best parts into languages each with its own cohesive vision.

cr. What are the other four eras of programming language evolution?

cr. Every language has been able to "take all of the learnings since the 1950s and 
incorporate the best parts into languages each with its own cohesive vision."

With tools like LLVM and the explosion of open source, creating a language has never been easier.

Rust has been voted the “most loved programming language” in the Stack Overflow Developer Survey every year since 2016.

If you work within that style then your code will flow naturally and the language will feel like it is working with you. On the other hand, if you fight the natural style of the language you will find it hard or impossible to express your ideas.

cr. I agree

Moreover, learning and working with a language will teach you ways to be more effective based on how the language guides you based on its natural design. How much you are able to learn is a function of how much your prior experience and mental models cover the new language.

cr. blah

The most unique feature of the language, the borrow checker, is a system that enforces certain invariants which allow you to make certain safety guarantees. Even this is built on prior art found in earlier languages.

cr. That's cool of Rust. What "art found in earlier languages" are you talking about? No reference, just talk.

On Language Comparison
There is no best programming language. Almost every task has a variety of languages which could be the right tool. Every language comes with good parts and bad parts.

cr. I agree

Language Features
--------------------------------------------------------------------------------
- Performance
- Strong, static, expressive type system • Great error messages
- Modern generics
- Memory safety
- Fearless concurrency • Cross platform
- C interoperability

Performance
--------------------------------------------------------------------------------
Rust is exceptionally fast, in the same ballpark as C and C++.
...
For the vast majority of use cases, you should consider Rust to be fast enough.

cr. Is this to imply that because Rust is adding extra checks it is adding some overhead
and can "never" compete with a pure c program?

Zero-cost abstractions, which are summarized by Bjarne Stroustrup, the creator of C++, as:
What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

Most of the abstractions in Rust, for example iterators, are zero-cost by this definition.
The most efficient way to traverse a vector of data is to use a for loop which uses an iterator trait.
The generated assembly is usually as good as you could hope for had you written it by hand.

cr. Is that to imply that the most efficient code you can write is assembly code by hand?

Rust does not have a garbage collector so you can use exactly as much memory as is strictly necessary at any given time. Due to the design of the language, you start to think and see every memory allocation. Using less memory is often easier than the converse. The rest of the language is designed around making working without a garbage collector painless.

cr. Cool!

Type System
--------------------------------------------------------------------------------
The type system of Rust is influenced by the long lineage of functional programming languages such as ML and Haskell. It is static, nominal, strong, and for the most part inferred. 

A type system is often called expressive if it is easy to encode your ideas.

There are some concepts which are impossible to express in static type systems. Rust has powerful abstraction facilities like sum and product types, tuples, generics, etc. which put the type system definitely in the expressive camp.

cr. What "concepts...are impossible to express?"

Memory Safety
--------------------------------------------------------------------------------
A language is memory safe if certain classes of bugs related to memory access are not possible.
A non-exhaustive list of memory related bugs include:
- dereferencing null pointers
- use-after free
- dangling pointers
- buffer overflows

Rust is designed to be memory safe, and thus it does not permit null pointers, dangling pointers, or data races in safe code.

The primary one is the unique system of ownership combined with the borrow checker. This is part of the compiler that ensures pieces of data live at least as long as they need to in order to be alive when they are used.

Option vs. Null
--------------------------------------------------------------------------------
One other feature is the builtin Option type. This is used to replace the concept of null found in many other languages. In some languages, every type is secretly the union of that type with null. This means that you can always end up with bugs where you assume some variable had a value and it actually was inhabited by the dreaded null. Rust disallows this by not having null and instead having a type which can explicitly wrap other types

```rust
fn print_number(num: Option<i32>) { match num {
    Some(n) => println!("I see {}!", n),
    None => println!("I see nothing!"),
  }
}
fn main() {
let x = Some(42); let y = None;
  print_number(x);
  print_number(y);
}
```

Unsafe
--------------------------------------------------------------------------------
The one caveat here is that Rust does allow blocks of code to be marked unsafe and within those blocks it is possible to violate memory safety. Some things are impossible for the compiler to verify are safe and therefore it refuses to do so. It requires you to use unsafe regions of code to ensure that you understand the invariants required to make sure your code truly is safe.

As an example, calling C functions from Rust is unsafe. This is because Rust has no way of knowing what the C code is doing, and C is inherently unsafe, therefore the compiler cannot uphold its guarantees if you call out to C. However, can it be safe to call C? Yes, provided you fill in the visibility gap for the compiler with your own logic.

Concurrency
--------------------------------------------------------------------------------
Concurrency in programming means that multiple tasks can be worked on at the same time. This is possible even for a single thread of execution by interleaving the work for different tasks in chunks rather than only working on tasks as entire chunks.
Parallelism in programming means multiple tasks executing at the exact same time. True parallelism requires multiple threads of execution (or the equivalent).

Go is designed around Communicating Sequential Processes (CSP) and therefore concur- rency is most easily achieved using channels and goroutines. Python, on the other hand, has libraries for threads, multiprocesses, message passing actors, etc.

Rust is a low-level language by design and therefore provides tools that allow you to use the model of your choice to achieve your particular goals. Therefore, there are facilities for threads but also channels and message passing.

Regardless of what technique you choose to tackle concurrency and/or parallelism, the same ownership model and type system that ensures memory safety also ensures thread safety. This means that it is a compile time error to write to the same memory from different threads without some form of synchronization.

C interoperability
--------------------------------------------------------------------------------
Rust is foremost a systems programming language, designed for building low level systems with strict performance requirements and reliability constraints.
C is the glue that binds many disparate systems.
It is straightforward to interact with C both by calling into C from Rust, as well as exposing Rust as a C library.

C is also the most common mechanism for making dynamic languages faster. Typically, when parts of your Python or Ruby code are showing performance problems, you can reach for an extension written in C to speed things up. Well, now you can write that extension in Rust and get all of the high level benefits of Rust and still make your Python or Ruby code think it is talking to C. This is also quite an interesting area for interacting with the JVM.

cr. Interested in more with JVM...replace Java?

There is an explosion of embedded systems due to what is commonly called the Internet of Things. However, is C still the best tool for that job? Mission critical software that controls real objects that could lead to serious consequences in the case of failure should be using the best tool for the job. Rust is a serious contender in this space.

One area in which Rust might not be right is when interfacing with large C++ codebases. It is possible to have C++ talk to C and then have C talk to Rust and vice versa. That is the approach you should take today if possible. However, Rust does not have a stable ABI nor a stable memory model. Hence, it is not directly compatible with C++. You can incrementally replace parts of a system with Rust and you can build new parts in Rust, but plug-and-play interoperability with C++ is not a solved problem.

Type systems are amazing yet there are valid valid programs which are inexpressible in a static type system.
dynamic languages can frequently be more productive for small, isolated tasks. Sometimes the cost of the type system is not worth it.

Shared, mutability state
--------------------------------------------------------------------------------
Shared, mutable state is the root of all evil.
Functional programming attacks that problem by doing away with mutability.
Rust attacks it by doing away with sharing.

Rustup
--------------------------------------------------------------------------------
The rustup7 tool is your one stop shop for managing multiple versions of the Rust compiler on your machine.
You can have different versions of the compiler installed next to each other and easily switch back and forth between them.

Cargo
--------------------------------------------------------------------------------
`rustc` is the Rust compiler, and you can invoke it directly, however you will find this rarely to be necessary as the majority of your time will be spent interacting with Cargo.
Cargo is a dependency manager and a build system.
You use a manifest to specify details of your code and its dependencies and you can then instruct Cargo to build your code and it will take care of the rest.
You can have Cargo manage building for other platforms and for quickly type checking via `cargo check`.
You use it to run tests via `cargo test` and for countless other tasks.

Clippy
--------------------------------------------------------------------------------
The linter is affectionately named Clippy.
Cargo supports an awesome feature where you can install subcommands via rustup so that you can selectively add components to Cargo based on your needs.
Clippy can be installed this way by running:
`cargo clippy`
It provides a bunch of helpful information and is good to run against your code regularly.

There are many ways to configure it both at a project level as well as at particular points in your code.

Rustfmt
--------------------------------------------------------------------------------
Rust has an official code formatter called rustfmt16.
This was a project that started life in the community and eventually got official status.
However, it is not as seriously official as `gofmt` for the Go language. 
You can configure rustfmt based on a couple attributes. 

Documentation
--------------------------------------------------------------------------------
All crates on crates.io will automatically have its documentation built and available on docs.rs
Rust has great facilities for including documentation in your code which is why most crates are quite well documented.
One excellent feature is the ability to include code samples in your documentation which is actually checked by the compiler. Thus the code examples in the documentation are never out of date

cr. Cool!

First App
================================================================================

`cargo new numbers`
Created binary (application) `numbers` package

Running `cargo new project_name` by default is equivalent to `cargo new project_name --bin`which generates a binary project. 

Alternatively, we could have run `cargo new project_name --lib` to generate a library project.

Binary vs. library
--------------------------------------------------------------------------------
Binary
A binary project is one which compiles into an executable file.
For binary projects, you can execute cargo run at the root of your application to compile and run the executable.

Library
A library project is one which compiles into an artifact which is shareable and can be used as a dependency in other projects.

Running cargo run in a library project will produce an error as cargo cannot figure out what executable you want it to run (because one does not exist).

Instead, you would run cargo build to build the library.

The default is to generate an rlib which is a format for use in other Rust projects.

main.rs
--------------------------------------------------------------------------------
For a binary project, the entry point is assumed to be located at `src/main.rs`.
Furthermore, inside that file, the Rust compiler looks for a function named `main` which will be executed when the binary is run.
Cargo has generated a main.rs file

Trade-offs
--------------------------------------------------------------------------------
Rust has no garbage collector and no runtime in the traditional sense.

cr. huh? as in JVM runtime? Chrome V8?

However, most difficulties of working with manual memory management are taken care of for you by the compiler.
Therefore, you will often hear “zero cost” being used to describe certain features or abstractions in the language and standard library.
This is meant to imply that neither performance nor reliability has to suffer to achieve a particular goal.
You write high level code and the compiler turns it into the same thing as the “best” low level implementation.


Making A Web App With Actix
--------------------------------------------------------------------------------

### Web Ecosystem
One area where Rust stands out is in the building of web servers.
Rust has its origins at Mozilla primarily as a tool for building a new browser engine.

There are a few different layers to the web programming stack. Primarily we are concerned here with the application layer which is comparable to where Django, Rails, and Express live in Python, Ruby, and NodeJS, respectively.

### Libraries that make up the web landscape

### Hyper
Hyper is a low level HTTP library built on even lower level libraries for building network services.
Currently most web frameworks use Hyper internally for handling the actual HTTP requests.


It can be used to build both HTTP clients and servers. However, there is a bit more boilerplate than you might want to write yourself when you want to focus on building an application. Therefore, we will use a library at a higher level of abstraction which still allows us to take advantage of what Hyper offers.

### Actix
The Actix project is actually a group of projects which define an actor system as well as a framework for building web applications. The web framework is aptly named actix-web. It has been built on top of futures and async primitives from the beginning. It also runs on the stable version of the compiler.

It recently hit the 1.0 milestone which should bring some much needed stability to the ecosystem. Additionally, it has been at the top of the Tech Empower web framework benchmarks30. Even if those are artificial benchmarks, it still points to the performance potential possible.

