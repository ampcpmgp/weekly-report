#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct OtherPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> OtherPoint<T, U> {
    fn mixup<V, W>(self, other: OtherPoint<V, W>) -> OtherPoint<T, W> {
        OtherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 5.1, y: 10.0 };

        println!("{:?} {:?}", integer, float);

        let integer_and_float = OtherPoint { x: 5, y: 4.0 };
        println!("{:?}", integer_and_float);

        println!("{}", float.x());
        println!("{}", float.distance_from_origin());

        let str_and_char = OtherPoint { x: "Hello", y: 'c' };
        println!("{:?}", str_and_char.mixup(integer_and_float));
    }
}
