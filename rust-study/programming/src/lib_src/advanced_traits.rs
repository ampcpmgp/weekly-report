// https://doc.rust-jp.rs/book/second-edition/ch19-03-advanced-traits.html

use std::fmt;
use std::ops::Add;
use std::option::Option;

#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        Option::Some(self.count)
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter { count: 3 };

    assert_eq!(4, counter.next().unwrap());
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn test_point() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[test]
fn test_millimeters() {
    assert_eq!(Millimeters(5) + Meters(1), Millimeters(1005));
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("captain");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("waving arms");
    }
}

#[test]
fn test_human() {
    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn test_animal() {
    println!("{}", Dog::baby_name());
    // println!("{}", Animal::baby_name()); // エラー!
    println!("{}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[test]
fn test_outline_print() {
    struct Point {
        x: i32,
        y: i32,
    };

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    Point { x: 3, y: -5 }.outline_print();
}

pub struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test]
fn test_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("{}", w);
}
