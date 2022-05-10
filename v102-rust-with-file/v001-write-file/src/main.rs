use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let mut f = File::create("hello.txt")?;
    f.write_all("Hello, world!".as_bytes())?;
    Ok(())
}
