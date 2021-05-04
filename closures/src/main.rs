use std::collections::HashMap;

struct Cacher<T> {
    caculation: T,
    value: HashMap<i32, i32>,
}

impl <T> Cacher<T> 
where T: Fn(i32) -> i32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            caculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: i32) -> i32 {
        match self.value.get(&arg) {
            Some(&x) => x,
            _ => {
                let x = (self.caculation)(arg);
                self.value.insert(arg, x);
                x
            }
        }
    }
}

#[test]
fn map_test() {
    let mut example = Cacher::new(|n| n);
    let _example1 = example.value(1);
    let example2 = example.value(2);
    assert_eq!(example2, example.value(2));
}

fn main() {
}
