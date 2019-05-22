// https://doc.rust-lang.org/book/ch15-01-box.html

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn run() {
    {
        let b = Box::new(5);
        println!("{}", b);
    }

    {
        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        println!("{:?}", list);
    }
}
