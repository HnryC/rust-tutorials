fn main() {
    //vectors
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(99); //must be mutable
    vec1.push(10);
    vec1.push(88);
    println!("The first vector contains {} and {}", vec1[0], vec1[1]);
    let greetings = vec!("Hello", "Goodbye", "Chao", "Bonjor");
    println!("Your greetings are {}, {}, {} and {}", greetings.get(0).unwrap(), greetings.get(1).unwrap(), greetings.get(2).unwrap(), greetings.get(3).unwrap());

    //iteration
    vec1.pop();
    for num in &mut vec1 {
        println!("i = {}", num);
        *num +=2;
        println!("i = {}", num);
    };
    for greeting in &greetings {
        println!("{}", greeting);
    }

    //errors
    let x = match greetings.get(4) { //doesn't crash
        Some(..) => true,
        None => false,
    };
    if !x {
        println!("Num out of range");
    } else {
        println!("Num in range");
    };
    let x = greetings[4]; //crashes
    println!("{}", x);
}
