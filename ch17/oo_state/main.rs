use oo_state::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.add_text(", but it was bad");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text(", and pizza for dinner");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "I ate a salad for lunch today, and pizza for dinner",
        post.content()
    );
}
