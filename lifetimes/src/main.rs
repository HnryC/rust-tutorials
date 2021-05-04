fn main() {
    let string1 = String::from("Hello world");
    {
        let string2 = "ioaresnt";
        let longest = longest(string1.as_str(), string2);
        println!("{}", longest);
    }
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() < string2.len() {
        string2
    } else {
        string1
    }
}
