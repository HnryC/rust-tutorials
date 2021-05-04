fn main() {
    println!("Hello, world!");
    let f = "Hello".to_string();
    let x_in_f = first_word(&f);
    println!("The first word is {}.", x_in_f);
    let y_in_f = second_word(&f);
    println!("The second word is {}.", y_in_f);
    let last_word = last_word(&f);
    println!("The last word is {}.", last_word);
}

fn first_word(sentence: &String) -> &str {
    let sentence_bites = sentence.as_bytes();
    for (i, &item) in sentence_bites.iter().enumerate() { //.enumerate returns the a tuple of the index and the current byte
        if item== b' ' {
            return &sentence[..i];
        }
    }
    &sentence[..]
}

fn second_word(sentence: &String) -> &str {
    let sentence_bites = sentence.as_bytes();
    let mut word_num = 0;
    let mut word_start = 0;
    for (i, &item) in sentence_bites.iter().enumerate() {
        if item == b' '{
            if word_num == 0 {
                word_num += 1;
                word_start = i + 1;
            }else {
                return &sentence[word_start..i];
            }
        }
    }
    if word_num == 0 {
        return "no second word";
    }
    &sentence[word_start..]
}

fn last_word(sentence: &String) -> &str {
    let sentence_bytes = sentence.as_bytes();
    let mut word_start = 0;
    for (i, &item) in sentence_bytes.iter().enumerate() {
        if item == b' ' {
            word_start = i + 1;
        }
    }
    &sentence[word_start..]
}
