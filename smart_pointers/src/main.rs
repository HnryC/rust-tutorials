use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

struct SmartPointer {
    data: String,
}

impl SmartPointer {
    fn new(data: &str) -> SmartPointer {
        SmartPointer {
            data: data.to_string()
        }
    }
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping data! {}", self.data);
    }
}

struct MyBox<T> (T);

impl <T> MyBox<T> {
    fn new(input: T) -> MyBox<T> {
        MyBox(input)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let list1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Cons(15, Rc::new(Nil)))))));
    let list2 = Rc::new(Cons(0, Rc::clone(&list1)));
    let list3 = Cons(0, Rc::clone(&list1));
    let list3 = Cons(-5, Rc::clone(&list2));
    println!("Num of lists containgin list 1: {}", Rc::strong_count(&list1));

    let a = SmartPointer::new("Last drop");
    let b = SmartPointer::new("Premedetated drop");
    let c = SmartPointer::new("First drop");
    let x = 3;
    let y = MyBox::new(3);
    assert_eq!(x, 3);
    assert_eq!(*y, 3);
    drop(b);
    println!("Program still running");
}
