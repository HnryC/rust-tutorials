fn main() {
    custom_fn(3, 5);
    let x = return_fn();
    let y = return_block();
    println!("x={}, y={}", x, y);
}

fn custom_fn(x: i32, y: i64) {
    println!("{}{b}", y, b = x);
}

fn return_fn() -> i32 {
    let x = 33;
    x
}

fn return_block() -> i32 {
    let x = 99;
    return x;
}
