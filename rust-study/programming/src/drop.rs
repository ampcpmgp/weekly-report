struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}", self.data);
    }
}

pub fn run() {
    {
        CustomSmartPointer {
            data: String::from("my stuff"),
        };

        CustomSmartPointer {
            data: String::from("other stuff"),
        };

        println!("created");
    }

    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };

        println!("created");
        drop(c);
        println!("dropped before the end of main.");
    }
}
