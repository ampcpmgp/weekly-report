// https://doc.rust-jp.rs/book/second-edition/ch18-01-all-the-places-for-patterns.html

pub fn run() {
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("{}", color);
        } else if is_tuesday {
            println!("{}", "tuesday");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("purple");
            } else {
                println!("orange");
            }
        } else {
            println!("blue");
        }
    }

    // while let 条件分岐ループ
    {
        println!("--- while let 条件分岐ループ ---");
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // for ループ
    {
        println!("--- for ループ ---");
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} {}", value, index);
        }
    }

    // 関数の引数
    {
        println!("--- 関数の引数 ---");
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("{} {}", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);
    }
}
