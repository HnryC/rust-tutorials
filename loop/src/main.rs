fn main() {
    println! ("{}", loop { // loops forever
        break 1; //breaks with return value
    } );
    let mut a = 9;
    while a > 0 {
        a-=1;
        println!("{}", a);
    }
    println!("{}", a);
    let x = [20, 30, 33, 69, 45, 6];
    for b in x.iter() {
        println!("{}", b);
    }
    for x in (0..10).step_by(2) { //starts at 0, steps by 2, dosen't include 10
        println!("{}", x);
    }
    for c in (1..10).step_by(2).rev() { //starts at 9, includes 1
        println!("{}", c);
    }
}
