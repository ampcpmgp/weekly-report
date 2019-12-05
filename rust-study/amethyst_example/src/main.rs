mod examples;
use amethyst;
use std::env;

fn main() -> amethyst::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("`src_all_api/examples` にあるmod名を引数に入れてください");
    }

    let name = args[1].as_str();

    match name {
        "simple_state" => examples::simple_state::run(),
        _ => panic!("`src_all_api/examples` のmod名と一致しません。"),
    }
}
