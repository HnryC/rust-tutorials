use std::process;
use std::env;
use mini_grep::*;


fn main() {
    let items = GrepItems::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error reading arguments: {}", err);
        process::exit(1);
    },);
    if let Err(e) = run(&items) {
        eprintln!("Program exited with error code {}", e);
        process::exit(2);
    }
}
