pub mod input {
    pub use std::io::{self, Write};
    pub fn input(message: String) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();
        get_word()
    }
    fn get_word() -> String {
        let mut temp = String::new();
        io::stdin()
        .read_line(&mut temp)
        .unwrap();
        temp
    }
}

pub mod output {
}
