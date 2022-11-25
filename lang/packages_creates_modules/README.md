# [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

- A package can contain multiple binary crates and optionally one library crate.
- As a package grows, you can extract parts into separate crates that become external dependencies.
- For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces, which we’ll cover in the “Cargo Workspaces” section in Chapter 14.
- You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.
- These features, sometimes collectively referred to as the module system, include:
  - Packages: A Cargo feature that lets you build, test, and share crates
  - Crates: A tree of modules that produces a library or executable
  - Modules and use: Let you control the organization, scope, and privacy of paths
  - Paths: A way of naming an item, such as a struct, function, or module

## [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

### Crate
- A crate is the smallest amount of code that the Rust compiler considers at a time.
- Even if you run `rustc` rather than `cargo` and pass a single source code file, the compiler considers that file to be a crate.
- Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
- A crate can come in one of two forms:
  1. binary crate
  2. library crate
  . All the crates we’ve created so far have been binary crates.
- The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate

#### Binary Crate
- Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.
- Each must have a function called `main` that defines what happens when the executable runs.

#### Library Crate
- Library crates don’t have a main function, and they don’t compile to an executable.
- They define functionality intended to be shared with multiple projects.
- Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".

### Package
- A package is a bundle of one or more crates that provides a set of functionality.
- A package contains a `Cargo.toml` file that describes how to build those crates.
- A package can contain as many binary crates as you like, but at most only one library crate.
- A package must contain at least one crate, whether that’s a library or binary crate.
- Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.
- Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.
- Cargo passes the crate root files to rustc to build the library or binary.
- If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a binary and a library, both with the same name as the package.
- A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.

## [Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#defining-modules-to-control-scope-and-privacy)

### [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following mod garden
  - In the file src/garden.rs
  - In the file src/garden/mod.rs
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
  - In the file src/garden/vegetables.rs
  - In the file src/garden/vegetables/mod.rs
- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- **Private vs public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write Asparagus to make use of that type in the scope.

### [Grouping Related Code in Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#grouping-related-code-in-modules)

- Modules let us organize code within a crate for readability and easy reuse.
- Modules also allow us to control the privacy of items, because code within a module is private by default.
- Private items are internal implementation details not available for outside use.
- We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
- We define a module with the `mod` keyword followed by the name of the module
- The body of the module then goes inside curly brackets.
- Inside modules, we can place other modules,
- In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
- If you want to make an item like a function or struct private, you put it in a module.
- Making the module public doesn’t make its contents public.
- The `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code.
- Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.
- The privacy rules apply to structs, enums, functions, and methods as well as modules.

## [Paths](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)

- Used to tell Rust where to find an item, similar to navigating a filesystem.
- A path can take two forms:
  1. An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal `crate`.
  2. A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.
- Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).
- Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, and depends on whether you’re more likely to move item definition code separately from or together with the code that uses the item.
- Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. To continue with our metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant they operate.

### [Starting Relative Paths with Super](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super)

- We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path.
- Using `super` allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent, but the parent might be moved elsewhere in the module tree someday.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

### [Making Structs and enums Public](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

- We can also use pub to designate structs and enums as public, but there are a few details extra to the usage of pub with structs and enums.
- If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private.
- We can make each field public or not on a case-by-case basis.

### [Using External Packages](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-external-packages)

- Add external packages to `Cargo.toml`
- Add use keyword to pull them into scope.

Note that the standard `std` library is also a crate that’s external to our package.
Because the standard library is shipped with the Rust language, we don’t need to change `Cargo.toml` to include `std`.
But we do need to refer to it with use to bring items from there into our package’s scope.

For example, with HashMap we would use this line:
```rust
use std::collections::HashMap;
```

To import multiple items from the same crate add comma separated list inside brakcets
```rust
use std::{cmp::Ordering, io};
```

The Glob Operator
- If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
```rust
use std::collections::*;
```

Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test into the tests module;

## [Separating Modules into Different Files]()

- Note that you only need to load a file using a mod declaration once in your module tree.
- Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared (e.g. absoulte path or relative path).
- In other words, mod is not an “include” operation that you may have seen in other programming languages.

### Alternate File Paths
- src/front_of_house.rs (what we covered)
- src/front_of_house/mod.rs (older style, still supported path)

- src/front_of_house/hosting.rs (what we covered)
- src/front_of_house/hosting/mod.rs (older style, still supported path)

The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.