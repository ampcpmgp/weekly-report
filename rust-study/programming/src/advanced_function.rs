fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn run() {
    {
        let answer = do_twice(add_one, 5);
        println!("{}", answer);
    }

    println!("クロージャとfnを受け入れるmapメソッド");
    {
        let list_of_numbers = vec![1, 2, 3];
        // このコードは、下にあるコードと同じようにコンパイルされる
        // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();

        println!("{:?}", list_of_strings);
    }

    println!("クロージャの返却");
    {
        fn returns_closure() -> Box<Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        println!("{}", returns_closure()(5));
    }
}
