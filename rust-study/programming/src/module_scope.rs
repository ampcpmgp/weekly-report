// https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html

mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                // Function body code goes here
            }
        }
    }

    mod voice {}
}

mod performance_group {
    pub use crate::module_scope::sound::instrument::woodwind;

    pub fn clarinet_trio() {
        woodwind::clarinet();
        woodwind::clarinet();
        woodwind::clarinet();
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }

        pub fn id(&self) -> i32 {
            self.id
        }
    }
}

mod menu {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn run() {
    crate::module_scope::sound::instrument::woodwind::clarinet();
    sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{}", v.name);

    // println!("{}", v.id); // error
    println!("{}", v.id());

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    println!("{:?} {:?}", order1, order2);

    use crate::module_scope::sound::instrument::woodwind;

    woodwind::clarinet();
    woodwind::clarinet();
    woodwind::clarinet();

    // これは有効だが、関数は親moduleをuseで指定するのが慣用的
    use crate::module_scope::sound::instrument::woodwind::clarinet;

    clarinet();
    clarinet();
    clarinet();

    // 構造体、列挙型等は、項目へのフルパスを指定するのが慣用的
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(1, 2);

    // 上記例外として、同じ名前を持つ2つの型を同じスコープなら、親モジュールを使う必要がある
    use std::fmt;
    use std::io;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let origin = Point { x: 0, y: 0 };

    println!("The origin is: {}", origin);

    fn get_string() -> io::Result<String> {
        Ok(String::from("aaa"))
    }

    println!("{:?}", get_string());

    // as キーワードを利用して名前を変更
    use std::io::Result as IoResult;

    fn get_string_io_result() -> IoResult<String> {
        Ok(String::from("bbb"))
    }

    println!("{:?}", get_string_io_result());

    // pub use を使うと外から使える
    performance_group::clarinet_trio();
    performance_group::woodwind::clarinet();

    // rand::Rangを利用
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("{}", secret_number);

    // 入れ子
    {
        use sound::{self, instrument};

        sound::instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
    }

    // Glob - 乱用禁止、テストで使う良い
    {
        use sound::instrument::woodwind::*;

        clarinet();
    }
}
