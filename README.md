# Kitten: A mini cat command 🐱

In this repo, we will ll build a command line tool using Rust.
As we go through every worksheet, a corresponding branch will be added here.
The branch will contain the source could you should have once you have finished the worksheet.
Note that the repo will only contain the source code (not the compiled executable).
You should thus build and execute the code using Cargo.
We recommend you install Cargo using [Rustup](https://rustup.rs#).
Then from your terminal run the following to compile the package:

```bash
cargo build
```

Now run the compiled package by running:

```bash
cargo run
```

## Stage 1: A basic structure

[Worksheet 0](https://docs.google.com/document/d/1-Whyq8QIRzTl00rSeeeO70deLPhB3_tEREvbEE2ALkE/edit?usp=sharing) guides you through the installation of Cargo and creation of this package.
Once you have finmished it, you should have the following files in your `kitten` directory.

- `src/main.rs` is a simple Hello World program written in Rust.
- `Cargo.toml` is the manifest file containing metadata about the package. For more information, checkout the [Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html).

## Sources and reading material

Worksheet 0 is mainly based on [The Rust Programming Language textbook](https://doc.rust-lang.org/book/).
We recommend reading the first chapter of this book at this stage.
