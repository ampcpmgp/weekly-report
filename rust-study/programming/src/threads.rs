use std::thread;
use std::time::Duration;

pub fn run() {
    {
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("{} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("{} from the main thred.", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    // スレッドにデータを渡す
    {
        println!("{}", "---スレッドにデータを渡す---");

        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("{:?}", v);
        });

        handle.join().unwrap();
    }
}
