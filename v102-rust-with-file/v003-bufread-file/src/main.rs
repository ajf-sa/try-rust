use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
fn main() -> Result<()> {
    let f = File::open("hello.txt")?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
