# how_to_rust

> "I know how to do this in Typescript. But how do I do it in Rust?"

This repo contains programs that describe how to do stuff in Rust that you know how to do in another language like TypeScript.

## First, some basics

### Install Rust
Install Rust using rustup by following the instruction at https://www.rust-lang.org

### Your first Rust file
Create, compile, run
```sh
touch hello.rs
echo "fn main() { println!(\"Hello computer\") }" > hello.rs
rustc hello.rs
./hello
```

### Your first Rust project
Create project with cargo
```sh
mkdir project_name
cd project_name
cargo init
```
Compile with cargo
```sh
cargo run
cargo build
./target/debug/project_name
cargo build --release
./target/release/project_name
```

## How to do X in Rustlang?


