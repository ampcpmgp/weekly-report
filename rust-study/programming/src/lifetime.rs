fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    // 以下のコードと同一
    // fn announce_and_return_part<'b>(&'b self, announcement: &str) -> &'b str {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}

pub fn run() {
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    {
        let novel = String::from("Call me. Some Years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        println!("{:?}", i);
        println!("{}", i.level());
        println!("{}", i.announce_and_return_part("test"));
    }

    {
        let s: &'static str = "something else";
        let n: &str = "something else";
        const S: &'static str = "something else";
        const N: &str = "something else";

        println!("{}", s);
        println!("{}", s == n);
        println!("{}", n == S);
        println!("{}", s == N);
    }

    {
        let string = String::from("hello");
        let static_1: &str = "aaa";
        let str_1: &str = &string;
        // let static_str_1: &'static str = &string; // error!

        let static_str_1: &'static str = &static_1;

        println!("{}", str_1);
        println!("{}", static_str_1);
    }
}
