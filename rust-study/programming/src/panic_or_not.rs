// https://doc.rust-lang.org/book/ch10-00-generics.html

use std::net::IpAddr;

pub fn run() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{}", home);

    {
        #[derive(Debug)]
        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("{}", value);
                }

                Guess { value }
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }

        let guess = Guess::new(5);

        println!("{}", guess.value());
    }
}
