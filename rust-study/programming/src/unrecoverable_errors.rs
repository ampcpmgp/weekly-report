// https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
pub fn run() {
    // panic!("crash and burn"); // crash and burn

    let v = vec![1, 2, 3];

    // buffer over-read
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
    // RUST_BACKTRACE=1 cargo run unrecoverable_errors
    v[99];
}
