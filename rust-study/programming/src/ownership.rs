// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

pub fn run() {
    let s = String::from("hello");
    let mut s2 = s;
    let s3 = s2.clone();

    s2.push_str(", world!");

    println!("{} {}", s2, s3);

    let x = 5;
    let y = x + 1;

    println!("x = {}, y = {}", x, y);

    let s = "string";
    let s2 = s;

    println!("s = {}, s2 = {}", s, s2);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("x = {}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{} {}", s1, s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("{} {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(mut some_integer: i32) {
    // some_integer comes into scope
    some_integer += 1;
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
