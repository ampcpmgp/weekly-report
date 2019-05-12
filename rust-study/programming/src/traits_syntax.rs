pub trait Summary: std::fmt::Debug {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, {} {}", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 以下と同一
// pub fn notify<T: Summary>(item: T) {
pub fn notify(item: impl Summary) {
    println!("{}", item.summarize());
}

// std::fmt::Display のエラー回避が不明
// pub fn notify_two<T: Summary + std::fmt::Display>(item: T, item2: T) {
//     println!("{} {}", item, item2);
// }

// where節の書き方 上記が分かり次第、下記を完成し、サンプルを作成する
// fn some_function<T, U>(t: T, u: U) -> i32
// where
//     T: std::fmt::Display + Clone,
//     U: Clone + std::fmt::Debug,
// {
// }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: true,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x = {}", self.x);
        } else {
            println!("y = {}", self.y);
        }
    }
}

pub fn run() {
    // traitを実装した型の利用
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course!"),
            reply: true,
            retweet: false,
        };

        println!("{}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
                hockey team in the NHL.",
            ),
        };

        println!("{}", article.summarize());
        notify(article);
        // notify_two(tweet, tweet);

        println!("{:?}", returns_summarizable());
    }

    // trait boundsによる関数の利用
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);

        println!("{}", result);

        let char_list = vec!['v', 'y', 'a'];
        let result = largest(&char_list);

        println!("{}", result);
    }

    // 条件付きでメソッドを実装するためのtrait boundsの利用
    {
        let pair = Pair::new(3, 6);
        pair.cmp_display();
    }
}
