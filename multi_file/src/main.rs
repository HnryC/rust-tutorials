mod custom_io;

fn main() {
    // cannot run (io::input::get_word();) because get_word is private
    let user_input = custom_io::input::input("Please enter a phrase: ".to_string());
    println!("You entered:  {}", user_input);
}
