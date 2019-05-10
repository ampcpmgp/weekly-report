// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn run() {
    let filepath = "data/hello.txt";
    let no_exist_filepath = "data/no_exist.txt";

    // ファイル基本操作
    {
        let f = File::open(filepath);
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(filepath) {
                    Ok(fc) => fc,
                    Err(error) => panic!("{:?}", error),
                },
                other_error => panic!("{:?}", other_error),
            },
        };

        println!("{:?}", f);
    }

    // ファイル基本操作の別な書き方
    {
        let f = File::open(filepath).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(filepath).unwrap_or_else(|error| {
                    panic!("{:?}", error);
                })
            } else {
                panic!("{:?}", error);
            }
        });

        println!("{:?}", f);
    }

    // エラー時のパニックのショートカット
    {
        // panic!
        // let f = File::open(no_exist_filepath).unwrap();

        // panic with message
        // let f = File::open(no_exist_filepath).expect("Failed to open");
    }

    // エラーの伝搬
    {
        fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
            let f = File::open(filepath);

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        let username = read_username_from_file(filepath);

        println!("{:?}", username);
    }

    // エラー伝搬のショートカット
    {
        fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
            let mut f = File::open(filepath)?;
            let mut s = String::new();

            f.read_to_string(&mut s)?;

            Ok(s)
        }

        let username = read_username_from_file(filepath);

        println!("{:?}", username);
    }

    // エラー伝搬のさらにショートカット
    {
        fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
            let mut s = String::new();

            File::open(filepath)?.read_to_string(&mut s)?;

            Ok(s)
        }

        let username = read_username_from_file(filepath);

        println!("{:?}", username);
    }

    // エラー伝搬の1行関数
    {
        fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
            fs::read_to_string(filepath)
        }

        println!("{:?}", read_username_from_file(filepath));
    }

    // 特性オブジェクト - あらゆる種類のエラー
    {
        fn file_open(filepath: &str) -> Result<(), Box<dyn Error>> {
            File::open(filepath)?;

            Ok(())
        }

        println!("{:?}", file_open(no_exist_filepath));
    }
}
