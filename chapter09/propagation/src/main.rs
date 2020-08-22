use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("username: {}", name),
        Err(err) => println!("ERROR: {:?}", err),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}