use std::process;
use std::env;
use mini_grep::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let items = GrepItems::new(&args).unwrap_or_else(|err| {
        eprintln!("Error reading arguments: {}", err);
        process::exit(1);
    },);
    if let Err(e) = run(&items) {
        eprintln!("Program exited with error code {}", e);
        process::exit(2);
    }
}
