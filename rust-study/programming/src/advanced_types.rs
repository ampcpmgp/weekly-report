pub fn run() {
    // 型エイリアス
    // type Thunk = Box<Fn() + Send + 'static>;

    // never型
    // fn bar() -> ! {}

    // Sized関数と、その制限を緩める関数
    // fn generic<T>(t: T) {}
    // fn generic<T: Sized>(t: T) {}
    // fn generic<T: ?Sized>(t: &T) {} // 緩める
}
