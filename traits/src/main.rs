mod lib;
use lib::Entities;

struct Person {
    x: i32,
    y: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Entities for Person {
}

impl Person {
    fn print(&self) {
        println!("x: {}, y:{}, x vel: {}, y vel: {}", self.x, self.y, self.x_vel, self.y_vel);
    }
}

fn main() {
    let player = return_entity();
    let player2 = Person { x: 3, y: 54, x_vel: 4, y_vel: 4 };
    player2.print();
    player2.update();
    player.update();
    let e = vec!['a', 'e', '^', '"', '\''];
    println!("{}", largest(&e));
    let f: [i8; 5] = [4, 8, 5, 2, 9];
    println!("{}", largest(&f));
    entities(&player2);
}

fn entities(entity: &impl Entities) {
    entity.update();
}

fn return_entity() -> impl Entities {
    Person {
        x: 5,
        y: 5,
        x_vel: 4,
        y_vel: 33,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
