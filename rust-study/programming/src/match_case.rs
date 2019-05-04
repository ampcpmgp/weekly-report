// https://doc.rust-lang.org/book/ch06-02-match.html

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn run() {
    println!("{}", value_in_certs(Coin::Penny));
    println!("{}", value_in_certs(Coin::Nickel));
    println!("{}", value_in_certs(Coin::Dime));
    println!("{}", value_in_certs(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_certs(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{} {} {}", five.unwrap(), six.unwrap(), none.unwrap_or(0));

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("one"),
        5 => println!("one"),
        _ => println!("one"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_certs(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
