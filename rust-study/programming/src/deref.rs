// https://doc.rust-lang.org/book/ch15-02-deref.html

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run() {
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x);
        let z = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *z);
    }

    // 関数とメソッドによる暗黙のDeref強制
    {
        let m = MyBox::new(String::from("Rust"));

        hello(&m);
        hello(&(*m)[..]);
    }
}
