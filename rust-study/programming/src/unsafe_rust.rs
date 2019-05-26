// https://doc.rust-jp.rs/book/second-edition/ch19-01-unsafe-rust.html#a%E7%94%9F%E3%83%9D%E3%82%A4%E3%83%B3%E3%82%BF%E3%82%92%E5%8F%82%E7%85%A7%E5%A4%96%E3%81%97%E3%81%99%E3%82%8B

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

pub fn run() {
    println!("参照から生ポインタを生成する");
    {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("{} {}", *r1, *r2);
        }
    }

    println!("unsafeな関数やメソッドを呼ぶ");
    {
        unsafe fn dangerous() {}

        unsafe { dangerous() }
    }

    println!("unsafeコードに安全な抽象を行う");
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        // let (a, b) = r.split_at_mut(3); // 以下と同様
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    println!("クラッシュが想定されるコード");
    {
        let address = 0x01345usize;
        let r = address as *mut i32;

        let _slice = unsafe { slice::from_raw_parts_mut(r, 10000) };

        // println!("{:?}", _slice); // クラッシュの確率が高い
    }

    println!("FFI の生成と使用");
    {
        extern "C" {
            fn abs(input: i32) -> i32; // ABIの定義
        }

        unsafe {
            println!("C: {}", abs(-3));
        }
    }

    println!("他の言語からRustの関数を呼び出す");
    {
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function");
        }
    }

    println!("可変で静的な変数にアクセスしたり、変更する");
    {
        static HELLO_WORLD: &str = "Hello, world!";

        println!("name is: {}", HELLO_WORLD);

        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            println!("{}", COUNTER);
        }
    }

    println!("unsafeなトレイトを実装する");
    {
        unsafe trait Foo {}
        unsafe impl Foo for i32 {}
    }
}
