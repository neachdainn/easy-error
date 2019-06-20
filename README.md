# Simple-error

This crate is a lightweight error handling library.
It does not attempt to do anything clever other than provide a simple error type that is meant for quick prototyping and a couple of methods for walking through the chain of errors.

## Example

```rust
use simple_error::{Error, ResultExt};

fn run(file: &str) -> Result<i32, Error> {
    let mut file = File::open(file).context("Could not open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Unable to read file")?;

    Ok(contents.trim().parse().context("Could not parse file")?)
}

fn main() {
    let file = "example.txt";

    if let Err(e) = run(file) {
        eprintln!("Error: {}", e);
        e.iter_causes().for_each(|c| eprintln!("Caused by: {}", c));
    }
}
```
