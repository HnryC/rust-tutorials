struct TestStruct {
    name: String,
    time: (i32, i32, i32),
    is_alive: bool
}

struct Color {
    red: i32,
    green: i32,
    blue: i32
}

impl Color {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        Color {
            red,
            green,
            blue
        }
    }
    fn get_color(&self, color: String) -> i32 {
        let color = color(..);
        match color {
            "red" => self.red
            _ => return 500
        }
    }
}

fn main() {
    let e: TestStruct = build_test("ienaste".to_string());
    let persons: [TestStruct; 3] = [TestStruct{
        name: "John Smith".to_string(),
        time: (11, 9, 01),
        ..e
    }, TestStruct {
        name: "James Bond".to_string(),
        time: (28, 2, 04),
        ..e
    }, TestStruct {
        name: "Bob Danials".to_string(),
        time: (4, 10, 33),
        ..e
    },];
    for person in &persons {
        print!("Name {}\n DOB {}/{}/{}\n living {}\n", person.name, person.time.0, person.time.1, person.time.2, person.is_alive);
    }
    let color: Color = Color::new(255, 255, 255);
    println!("{}, {}, {}", color.red, color.get_color("green"), color.get_color("blue"));
}

fn build_test(name: String) -> TestStruct {
    TestStruct {
        name,
        time: (10, 10, 10),
        is_alive: true
    }
}
