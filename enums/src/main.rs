use rand::Rng;

enum State {
    Idle,
    Run(i32),
    Fall(i32, i32),
}

impl State {
    fn new() -> Option<Self> {
        match rand::thread_rng().gen_range(0, 3) {
            0 => Some(State::Idle),
            1 => Some(State::Run(5)),
            2 => Some(State::Fall(3, 5)),
            _ => None,
        }
    }
}

fn main() {
    let state = State::new().unwrap();
    match state {
        State::Run(e) => println!("The character is running at {} meters per second", e),
        State::Fall(curr, max) => { print!("The character is falling");
            println!("at {} of {}", curr, max);
        },
        _ => println!("The character is in idle"),
    }
    if let State::Idle = state {
        println!("You should do something");
    } else {
        println!("You are doing someting");
    }
}
