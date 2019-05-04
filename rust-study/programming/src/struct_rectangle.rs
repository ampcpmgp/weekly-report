// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#methods-with-more-parameters

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // (&self).calc() と同様
        self.calc()
    }

    fn calc(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 分けて設定可能
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("{}", area(&rect1));
    println!("{}", rect1.area());
    println!("{}", rect2.can_hold(&rect1));
    println!("{:?}", Rectangle::square(10));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
