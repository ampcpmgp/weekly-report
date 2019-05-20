// https://doc.rust-lang.org/book/ch13-02-iterators.html

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn run() {
    {
        let v1 = vec![12, 3, 4];
        let v1_iter = v1.iter();

        for &val in v1_iter {
            println!("{}", &val);
        }
    }

    {
        let v1 = vec![12, 3, 4];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&12));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), None);
    }

    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| {
                println!("{} {}", a, b);
                a * b
            })
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}
