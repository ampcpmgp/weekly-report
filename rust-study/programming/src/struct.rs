// https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-and-instantiating-structs

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("test@example.com"),
        username: String::from("test123"),
        ..user1
    };

    let red = Color(255, 0, 0);

    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{:?}", red);
}
