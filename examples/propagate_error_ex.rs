#![allow(unused_variables)]
#![allow(unused_imports)]
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let greeting_file = File::open("../doc/cargo-issues/hello.txt")?;

    Ok(())
}
