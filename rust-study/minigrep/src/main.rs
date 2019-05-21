use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: env::Args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("{}", e);

        process::exit(1);
    }
}
