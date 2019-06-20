# Easy-error

This crate is a lightweight error handling library meant to play well with the standard `Error` trait.
There are three major components of this crate:

1. A basic, string-based error type that is meant for either quick prototyping or human-facing errors.
2. A nice way to iterate over the causes of an error.
3. Some macros that make returning errors slightly more ergonomic.

## Rust Version Requirements

The current version requires **Rustc 1.32 or newer**.
In general, this crate will be compilable with the Rustc version available on the oldest Ubuntu LTS release.
Any change that requires a new Rustc version will be considered a breaking change and will be handled accordingly.

## Example

```rust
use std::{fs::File, io::Read};
use easy_error::{Error, ResultExt};

fn run(file: &str) -> Result<i32, Error> {
    let mut file = File::open(file).context("Could not open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Unable to read file")?;

    let value = contents.trim().parse().context("Could not parse file")?;
    ensure!(value != 0, "Value cannot be zero");

    Ok(value)
}

fn main() {
    let file = "example.txt";

    if let Err(e) = run(file) {
        eprintln!("Error: {}", e);
        e.iter_causes().for_each(|c| eprintln!("Caused by: {}", c));
    }
}
```
