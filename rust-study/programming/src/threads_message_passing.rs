// https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("{}", received);

        thread::sleep(Duration::from_millis(100));
    }

    // 複数の値を送信し、受信側が待機するのを確かめる
    {
        println!("{}", "----");

        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            // 君のためにもっとメッセージを(more messages for you)
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("{}", received);
        }

        thread::sleep(Duration::from_millis(4200));
    }
}
