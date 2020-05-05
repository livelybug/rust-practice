pub mod talk {
    pub mod cat;
    pub mod dog;
}

#[test]
#[should_panic]
#[ignore]
fn test_hello() {
    assert_eq!("cat hello--", talk::cat::hello());
}