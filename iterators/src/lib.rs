struct Counter {
    val: i8
}

impl Counter {
    fn new() -> Counter {
        Counter { val: 0 }
    }
}

impl Iterator for Counter {
    type Item = i8;

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.val < 5 {
            self.val += 1;
            Some(self.val)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

impl Shoe {
    fn new(size: i32, style: &str) -> Shoe{
        Shoe {
            size,
            style: style.to_string(),
        }
    }
}

fn size_check(stock: Vec<Shoe>, size: i32) -> Vec<Shoe> {
    stock.into_iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let stock = vec![Shoe::new(10, "Traners"), Shoe::new(13, "Chelsea Boots"), Shoe::new(10, "Ankle Boots")];
        let can_wear = size_check(stock, 10);
        assert_eq!(vec![Shoe::new(10, "Traners"), Shoe::new(10, "Ankle Boots")], can_wear)
    }
}
