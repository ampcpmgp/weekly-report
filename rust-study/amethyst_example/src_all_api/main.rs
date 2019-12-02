mod examples;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("`src_all_api/examples` にあるmod名を引数に入れてください");

        return;
    }

    let name = args[1].as_str();

    match name {
        "simple_state" => examples::simple_state::run(),
        _ => println!("src配下にあるmod名を引数に入れてください"),
    }
}
