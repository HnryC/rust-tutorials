use std::collections::HashMap;

fn main() {
    let teams = vec!["Red".to_string(), String::from("Green")];
    let initial_scores = vec![14, 20];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    match scores.get("Blue") {
        Some(value) => println!("Blue has a value of {}", value),
        _ => println!("Blue not found"),
    }
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    match scores.get("Blue") {
        Some(value) => println!("Blue has a value of {}", value),
        _ => println!("Blue not found"),
    }
    for (key, value) in scores {
        println!("{} has the value of {}", key, value);
    }
}
