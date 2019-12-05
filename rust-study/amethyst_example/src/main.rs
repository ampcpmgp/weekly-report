mod examples;
use amethyst::LoggerConfig;
use std::env;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("`src_all_api/examples` にあるmod名を引数に入れてください");
    }

    let name = args[1].as_str();

    match name {
        "empty_state" => examples::empty_state::run(),
        "simple_state" => examples::simple_state::run(),
        "state" => examples::state::run(),
        "window" => examples::window::run(),
        _ => panic!("`src_all_api/examples` のmod名と一致しません。"),
    }
}
