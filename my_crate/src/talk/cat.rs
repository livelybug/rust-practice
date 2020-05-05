pub fn hello() -> String {"cat hello".to_string()}

#[test]
fn test_hello() {
    assert_eq!("cat hello", hello());
}