use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;
use std::fs;

fn main() {
    let username = read_username().unwrap_or_else(|error|{ panic!("Error reading username {:?}", error)});
    let password = match File::open("hello.jpg") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.jpg") {
                Ok(file) => file,
                Err(cr_error) => panic!("Problem opening the file {}", cr_error),
            }
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
    let pin = fs::read_to_string("pin.bin").unwrap();
}

fn read_username() -> Result<String, io::Error> {
    let mut str = String::new();
    File::open("name.txt")?.read_to_string(&mut str)?;
    Ok(str)
}
