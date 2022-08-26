# [Yew + Trunk](https://yew.rs/docs/getting-started/project-setup/using-trunk)

## Install

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

## New workspace
```bash
cargo new hello_trunk
```

## Add deps

```bash
cargo add yew
```


1. Bare bones:

src/
  main.rs
Cargo.toml
index.html

Run `trunk serve`

Build `trunk build --release`
