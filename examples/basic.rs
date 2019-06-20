use std::{fs::File, io::Read};
use easy_error::{Error, ResultExt};

fn run() -> Result<i32, Error> {
    let file = std::env::args().nth(1).unwrap_or("example.txt");
    let mut file = File::open(file).context("Could not open file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Unable to read file")?;

    let value = contents.trim().parse().context("Could not parse file")?;
    ensure!(value != 0, "Value cannot be zero");

    Ok(value)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        e.iter_causes().for_each(|c| eprintln!("Caused by: {}", c));
    }
}
