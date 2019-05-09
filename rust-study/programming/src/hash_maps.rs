use std::collections::HashMap;

pub fn run() {
    // 新しいハッシュマップを作成する
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("{:?}", scores);
    }

    // タプルのベクトルに対してcollectメソッドを使用する
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![20, 30];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        println!("{:?}", scores);
    }

    // ハッシュマップと所有権
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Favorite Blue");

        let mut map = HashMap::new();

        map.insert(field_name, field_value);

        println!("{:?}", map);
        // println!("{}", field_name); // error: value borrowed here after move
    }

    // ハッシュマップ内の値にアクセスする
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_score = String::from("Blue");
        let score = scores.get(&team_score);

        println!("{:?}", score);
    }

    // forループを使用して、ベクターと同じ方法でハッシュマップの各キー/値ペアを反復
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    // 値を上書きする
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    // キーに値がない場合にのみ値を挿入する
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    // 古い値に基づいて値を更新する
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
