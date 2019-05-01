fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1); と同義
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // println!("Values {} {} {}", tup.0, tup.1, tup.2); と同義
    println!("Values {} {} {}", x, y, z);
}
