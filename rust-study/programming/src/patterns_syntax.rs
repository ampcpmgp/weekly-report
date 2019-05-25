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

    println!("--- ネストされた_で値の一部を無視する ---");
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite");
            }

            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("{:?}", setting_value);
    }

    println!("_ と _x 記法の違い。");
    {
        let s = Some(String::from("Hello"));

        // _s を使うと、所有権を奪うため、 s が使えなくなる
        // if let Some(_s) = s {
        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    println!(".. で値の残りの部分を無視する");
    {
        #[allow(dead_code)]
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }

    println!("ref と ref mut でパターンに参照を生成する");
    {
        let robot_name = Some(String::from("Bors"));

        match robot_name {
            Some(ref name) => println!("{}", name),
            None => (),
        }

        println!("{:?}", robot_name);

        let mut robot_name = Some(String::from("Bors"));

        match robot_name {
            Some(ref mut name) => *name = String::from("Another name"),
            None => (),
        }

        println!("{:?}", robot_name);
    }

    println!("マッチガードで追加の条件式");
    {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than file: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("50"),
            Some(n) if n == y => println!("Matched, {:?}", x),
            _ => println!("Default"),
        }

        println!("x = {:?}, y = {:?}", x, y);

        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    println!("--- @束縛 ---");
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3...7,
            } => println!("@ 3...7 range {}", id_variable),
            Message::Hello { id: 10...12 } => println!("Found"),
            Message::Hello { id } => println!("{}", id),
        }
    }
}
