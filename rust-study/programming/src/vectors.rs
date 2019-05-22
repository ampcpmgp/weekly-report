// https://doc.rust-lang.org/book/ch08-01-vectors.html

pub fn run() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    println!("{:?}", v);

    let v = vec![1, 2, 3];

    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("{}", third);

    match v.get(2) {
        Some(third) => println!("third element {}", third),
        None => println!("no third element"),
    }

    let does_not_exist = v.get(100);

    println!("{:?}", does_not_exist);

    let v = vec![1, 2, 3];
    let first = &v[0];

    // ベクトルの末尾に新しい要素を追加すると、すべての要素を隣に配置するのに十分なスペースがない場合、新しいメモリを割り当てて古い要素を新しいスペースにコピーする必要があります。ベクトルが現在ある場所。その場合、最初の要素への参照は、割り当て解除されたメモリを指すことになります。借入ルールは、プログラムがそのような状況に陥るのを防ぎます。
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(6); // error!

    println!("{}", first);

    let v = vec![10, 3, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![10, 3, 5];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
