pub mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) { // &mut self: update internal state
            self.content.push_str(text); // content is in Post struct, decouple with state
        }

        pub fn content(&self) -> &str {
            // &self is post passed to delegate method content in state
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                // s.request_review() will return the new state per dynamic type draft/pendingReview implementation
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        // the return str part of post, so it should be stay as long as post
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            "" // default value for draft/pendingReview state
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        // self: Box<Self>
        // This syntax means the method is only valid when called on a Box holding the type. This
        // syntax takes ownership of Box<Self>, invalidating the old state so the state value of
        // the Post can transform into a new state.
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self // self is type of Box<Self> so match the return type
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {}) // returns a new, boxed instance of the Published struct.
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self // a published post should stay in Published if call request_review
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self // a published post should stay in Published if call approve
        }

        // if it's published, then return the actual content in post
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}
