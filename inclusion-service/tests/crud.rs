mod common;

#[test]
fn test_crud() {
    let client = common::Client::default();
    let stack = client.new_stack();
    assert!(stack.articlesdb_url().starts_with("postgres://rusty:rusty"))
}
