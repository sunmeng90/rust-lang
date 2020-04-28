use list17_19::blog::DraftPost;

// no pure object oriented, state change no longer encapsulated entirely within the post
// However, our gain is that invalid states are now impossible because of the type system and
// the type checking that happens at compile time!
// State is encoded into type DraftPost, PendingReviewPost, Post
fn main() {
    let mut post = DraftPost::new(); // DraftPost
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review(); // PendingReviewPost
    let post = post.approve(); // Post
    assert_eq!("I ate a salad for lunch today", post.content());
}
