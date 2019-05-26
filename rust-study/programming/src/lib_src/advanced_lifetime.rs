// https://doc.rust-jp.rs/book/second-edition/ch19-02-advanced-lifetimes.html

struct Context<'s>(&'s str);

// struct Parser<'c, 's> { // ドキュメントではこの書き方はエラー
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// struct Ref<'a, T>(&'a T); // ドキュメントではこの書き方はエラー

struct Ref<'a, T: 'a>(&'a T);

struct StaticRef<T: 'static>(&'static T);

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

#[test]
fn test() {
    let num = 5;
    let _obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}
