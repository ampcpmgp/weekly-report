// https://doc.rust-jp.rs/book/second-edition/ch15-06-reference-cycles.html

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use List::{Cons, Nil};

pub fn run() {
    {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("{} {:?}", Rc::strong_count(&a), a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!(
            "{} {} {:?}",
            Rc::strong_count(&a),
            Rc::strong_count(&b),
            b.tail()
        );

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("{} {}", Rc::strong_count(&b), Rc::strong_count(&a));

        // println!("{:?}", a.tail()); // 循環参照エラー
    }

    // 木データ構造を作る
    {
        println!("--- 木データ構造を作る ---");

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("{:?}", leaf.parent.borrow().upgrade());
        println!("{} {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("{:?}", leaf);
            println!("{:?}", branch);

            println!("{} {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
            println!("{} {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        }

        println!("{:?}", leaf.parent.borrow().upgrade());
        println!("{} {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}
