use easy_error::{ensure, ResultExt, Terminator};
use std::{fs::File, io::Read};

fn main() -> Result<(), Terminator> {
    let file = std::env::args().nth(1).unwrap_or("example.txt".to_string());
    let mut file = File::open(file).context("Could not open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Unable to read file")?;

    let value: i32 = contents.trim().parse().context("Could not parse file")?;
    ensure!(value != 0, "Value cannot be zero");

    println!("Value = {}", value);
    Ok(())
}
