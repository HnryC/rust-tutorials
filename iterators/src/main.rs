fn main() {
    let v1 = vec![1, 2, 3];
    let v1_itr = v1.iter();
    for val in v1_itr {
        println!("Got: {}", val);
    }
}

#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_itr = v1.iter();
    assert_eq!(v1_itr.next(), Some(&1));
    assert_eq!(v1_itr.next(), Some(&2));
    assert_eq!(v1_itr.next(), Some(&3));
    assert_eq!(v1_itr.next(), None);
}
