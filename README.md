# Easy-error

[![docs.rs](https://docs.rs/easy-error/badge.svg)](https://docs.rs/easy-error)
[![crates.io](http://img.shields.io/crates/v/easy-error.svg)](http://crates.io/crates/easy-error)
![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rustc 1.32+](https://img.shields.io/badge/rustc-1.32+-lightgray.svg)
![Pipeline](https://gitlab.com/neachdainn/easy-error/badges/master/pipeline.svg)

This crate is a lightweight error handling library meant to play well with the standard `Error` trait.
It is designed for quick prototyping or for Command-line applications where any error will simply bubble up to the user.
There are four major components of this crate:

1. A basic, string-based error type that is meant for either quick prototyping or human-facing errors.
2. A nice way to iterate over the causes of an error.
3. Some macros that make returning errors slightly more ergonomic.
4. A "termination" type that produces nicely formatted error messages when returned from the `main` function.

## Rust Version Requirements

The current version requires **Rustc 1.32 or newer**.
In general, this crate will be compilable with the Rustc version available on the oldest Ubuntu LTS release.
Any change that requires a new Rustc version will be considered a breaking change and will be handled accordingly.

## Example

```rust
use std::{fs::File, io::Read};
use easy_error::{ResultExt, termination};

fn main() -> termination::Result {
    let file_name = "example.txt";
    let mut file = File::open(file_name).context("Could not open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Unable to read file")?;

    let value: i32 = contents.trim().parse().context("Could not parse file")?;
    println!("Value = {}", value);

    Ok(())
}
```
