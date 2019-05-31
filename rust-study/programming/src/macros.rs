// https://doc.rust-jp.rs/book/second-edition/appendix-04-macros.html

extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Packages;

pub fn run() {
    Packages::hello_macro();
}
