pub fn zero_to_one_hundred(value: i32) {
    if value < 0 {
        panic!("ERR: Value too small, minimum = 0, given = {}", value);
    } else if value > 100 {
        panic!("ERR: Value too large, minimum = 100, given = {}", value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "ERR: Value too large")]
    fn input_too_large() {
        crate::zero_to_one_hundred(444);
    }
}
