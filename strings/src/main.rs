fn main() {
    let mut s = String::from("Hello");
    let s1 = " world";
    let s2 = " in toki pona is ".to_string();
    let s3 = String::from("toki e ma jan");
    s.push_str(s1); // Doesn't take ownership
    println!("{}", s1);
    let s4 = s + &s2 + &s3; //s goes out of scope
    println!("{}", s4);
    let s = format!("Hello{}{}{}", s1, s2, s3);
    println!("{}", s);
    if s == s4 {
        println!("Hello");
    }
}
