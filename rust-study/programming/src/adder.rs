pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail"); // error!
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(3, add_two(2));
    }
}
