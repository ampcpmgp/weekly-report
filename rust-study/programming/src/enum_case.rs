// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

use std::net::Ipv6Addr;

#[derive(Debug)]
enum Message {
    Quit,

    Move { x: i32, y: i32 },
    Write(String),

    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call() {
        println!("hello");
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn run() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 標準ライブラリにもある
    let std_ip = std::net::IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("{:?}", home);
    println!("{:?}", loopback);
    println!("{}", std_ip);

    // Option<T> 型はpreludeで読まれる
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{} {}", some_number.unwrap(), some_string.unwrap());

    if absent_number == None {
        println!("absent_number none");
    }

    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Move { x: 1, y: 2 });
    println!("{:?}", Message::Write(String::from("hello")));
    println!("{:?}", Message::ChangeColor(0, 0, 0));
    Message::call();
}
