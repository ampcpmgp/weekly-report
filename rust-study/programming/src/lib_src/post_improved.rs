// https://doc.rust-jp.rs/book/second-edition/ch17-03-oo-design-patterns.html#a%E7%8A%B6%E6%85%8B%E3%81%A8%E6%8C%AF%E3%82%8B%E8%88%9E%E3%81%84%E3%82%92%E5%9E%8B%E3%81%A8%E3%81%97%E3%81%A6%E3%82%B3%E3%83%BC%E3%83%89%E5%8C%96%E3%81%99%E3%82%8B

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
