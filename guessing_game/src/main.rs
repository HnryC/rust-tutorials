use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let answer = rand::thread_rng().gen_range(1, 100);
    println!("Guess a number");

    loop {
        let mut guess = String::new();
        get_word(&mut guess);
        let guess: u32 = match guess.trim().parse() { //shadowing a varable lets it be 
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&answer) { //match == switch, Ordering == enum
            Ordering::Less => println!("Your guess was too low"),
            Ordering::Equal => {
                println!("You are correct!"); 
                break;
            }
            Ordering::Greater => println!("Your guess was too high"),
        }
    }
}

fn get_word(guess: &mut String) -> &str {
    io::stdin()
        .read_line(guess)
        .unwrap()
}
