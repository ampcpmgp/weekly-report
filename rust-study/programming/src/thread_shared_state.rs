// https://doc.rust-jp.rs/book/second-edition/ch16-03-shared-state.html

use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("{:?}", m);
    }

    // 複数のスレッド間でMutex<T>を共有する
    {
        println!(
            "{}",
            "--- 複数のスレッド間でMutex<T>を共有する ---"
        );

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("{}", *counter.lock().unwrap());
    }
}
