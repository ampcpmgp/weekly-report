pub fn run() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('1');

    println!("{:?}", s);

    let s2 = String::from("world!");
    let s3 = s + &s2;

    println!("{:?}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;

    println!("{:?}", s4);

    let s1 = String::from("tic");
    let s4 = format!("{}-{}-{}", s1, s2, s3);

    println!("{:?}", s4);

    // let h = s1[0]; // Error!
    let hello = "Здравствуйте";
    let len = String::from(hello).len();
    let s = &hello[0..4];

    println!("{}", len);
    println!("{}", s);

    for c in hello.chars() {
        print!("{} ", c);
    }

    println!("");

    for b in hello.bytes() {
        print!("{} ", b);
    }
}
