pub mod blog {
    pub struct Post {
        content: String,
    }

    // encode the state to type
    pub struct DraftPost {
        content: String, // duplicate
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost { content: String::new() }
        }

        pub fn content(&self) -> &str {
            &self.content // & here is for content field
        }
    }

    impl DraftPost {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        //self: we want to take the ownership of (draftPost) and transform to PendingReviewPost
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content
            }
        }
    }

    pub struct PendingReviewPost {
        content: String, // duplicate again
    }

    impl PendingReviewPost {
        //self: again take ownership of pendingReviewPost and transform to published Post
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}