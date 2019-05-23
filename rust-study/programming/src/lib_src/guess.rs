pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("{}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[should_panic(expected = "105")]
    #[test]
    fn greater_than_100_with_expected_message() {
        Guess::new(105);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("not ok"))
        }
    }
}
