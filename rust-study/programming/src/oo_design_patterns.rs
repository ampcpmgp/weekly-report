// https://doc.rust-jp.rs/book/second-edition/ch17-03-oo-design-patterns.html

pub fn run() {
    {
        use study_lib::lib_src::post::Post;

        let mut post = Post::new();
        post.add_text("I ate a salad");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad", post.content());
    }

    // Rustの型検査システムを利用して状態と振る舞いを型化する
    {
        use study_lib::lib_src::post_improved::Post;

        let mut post = Post::new();

        post.add_text("I ate a salad");

        let post = post.request_review();
        let post = post.approve();

        assert_eq!("I ate a salad", post.content());
    }
}
