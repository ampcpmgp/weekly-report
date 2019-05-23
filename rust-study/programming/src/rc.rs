// https://doc.rust-lang.org/book/ch15-04-rc.html

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::rc::List::{Cons, Nil};

pub fn run() {
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        println!("{:?} {:?} {:?}", a, b, c);
    }

    // 参照カウントの確認
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("{}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));

        {
            let c = Cons(4, Rc::clone(&a));
            println!("{}", Rc::strong_count(&a));
            println!("{:?} {:?} {:?}", a, b, c);
        }

        println!("{}", Rc::strong_count(&a));
    }
}
