pub fn run() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("{} {} / {}", s1, len, no_dangle());
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s // &s だとダングリングポインタのためエラー
}
