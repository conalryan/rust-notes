# [Rust](https://www.rust-lang.org)

## Install
Rustup: the Rust installer and version management tool
```bash
curl https://sh.rustup.rs -sSf | sh
```

## Toolchain Management `rustup`
- Rapid 6-week release cycle
- `rustup update`

## Configuring the PATH environment variable
- All tools are installed to the `~/.cargo/bin` directory (Rust toolchain: including rustc, cargo, and rustup).
- Include this directory in their `PATH` environment variable. 

## Test installation
`rustc --version`

## Cargo: the Rust build tool and package manager
When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:
- build your project with `cargo build`
- run your project with `cargo run`
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to [crates.io](https://crates.io/) with `cargo publish`
- To test that you have Rust and Cargo installed, you can run this in your terminal of choice: `cargo --version`

## Code formatting tool Rustfmt
```bash
rustup component add rustfmt
```

## Linting tool Clippy
```bash
rustup component add clippy
```

