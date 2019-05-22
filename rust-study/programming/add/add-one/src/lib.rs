extern crate rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn random() -> i32 {
    rand::random()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn random_test() {
        let random = random();
        assert!(0 < random);
    }
}
