// https://doc.rust-lang.org/book/ch04-03-slices.html

fn main() {
    let s = String::from("hello world");
    let w = first_word(&s);

    // s.clear(); // Error!

    println!("{}", w);

    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];

    println!("{} {} {}", hello, world, whole);

    let string_literal = "hello world";
    let w = first_word(string_literal);

    println!("{}", w);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
