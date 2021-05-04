fn main() {
    let hello = String::from("hello"); //Since strings can have a variable length so are part of heap
    let mut other_hello = hello.clone();
    let x = 5;
    let y = x; //clone is not nesacary due to int's being pushed to the stack
    take_ownership(hello); //The hello varable cannot be called after theis point
    keep_ownership(y); //Y can be called ofter this point due to it being on the stack instead of the heap
    keep_string_ownership(&mut other_hello);
    println!("{}", other_hello);
    println!("{}", y);
}

fn take_ownership(word :String) {
    println!("{}", word);
}

fn keep_ownership(word :u32) {
    println!("{}", word);
}

fn keep_string_ownership(word :&mut String) {
    &word.push_str("hello");
    println!("{}", word);
}
