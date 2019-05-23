use study_lib;
mod common;

#[test]
fn it_adds_two() {
    // `cargo test -- --nocapture` を入力することで下記が出力される。
    println!("{}", "nocapture");
    assert_eq!(4, study_lib::lib_src::adder::add_two(2));
}

// `cargo test -- --ignored` で以下ignore annotationがついたもののみをテスト可能。
#[ignore]
#[test]
fn expensive_test() {
    common::setup();
    assert_eq!(4, 4);
}
