// https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html#a%E5%90%8D%E5%89%8D%E4%BB%98%E3%81%8D%E5%A4%89%E6%95%B0%E3%81%AB%E3%83%9E%E3%83%83%E3%83%81%E3%81%99%E3%82%8B

struct Point {
    x: i32,
    y: i32,
}

pub fn run() {
    {
        let x = 1;

        match x {
            1 => println!("one"),      // 1
            2 => println!("two"),      // 2
            3 => println!("three"),    // 3
            _ => println!("anything"), // なんでも
        }
    }

    // 名前付き変数にマッチする
    {
        println!("--- 名前付き変数にマッチする ---");

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("50"),
            Some(y) => println!("{:?}", y),
            _ => println!("{:?}", x),
        }

        println!("{:?} {:?}", x, y);
    }

    // 複数のパターン
    {
        println!("--- 複数のパターン ---");

        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // ... で値の範囲に合致させる
    {
        println!("--- ... で値の範囲に合致させる ---");

        let x = 5;

        match x {
            // 1から5まで
            1...5 => println!("one through five"),
            // それ以外
            _ => println!("something else"),
        }

        let x = 'c';

        match x {
            'a'...'j' => println!("early ASCII letter"),
            'k'...'z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    // 構造体を分配する
    {
        println!("--- 構造体を分配する ---");

        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;

        assert_eq!(0, a);
        assert_eq!(7, b);

        let Point { x, y } = p;

        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point { x, y: 0 } => println!("x {}", x),
            Point { x: 0, y } => println!("y {}", y),
            Point { x, y } => println!("{} {}", x, y),
        }
    }

    // enum を分配する
    {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move {} {}", x, y),
            Message::Write(text) => println!("{}", text),
            Message::ChangeColor(r, g, b) => println!("{} {} {}", r, g, b),
        }
    }

    // 参照を分配する
    {
        println!("--- 参照を分配する ---");

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];

        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

        println!("{}", sum_of_squares);
    }

    // 構造体とタプルを分配する
    {
        println!("--- 構造体とタプルを分配する ---");

        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

        println!("{} {} {} {}", feet, inches, x, y);
    }

    // _で値全体を無視する
    {
        println!("--- _で値全体を無視する ---");

        fn foo(_: i32, y: i32) {
            println!("{}", y);
        }

        foo(3, 4);
    }

    // ネストされた_で値の一部を無視する
    {
        println!("--- ネストされた_で値の一部を無視する ---");

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite");
            }
        }
    }
}
