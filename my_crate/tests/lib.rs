#[cfg(test)]
mod test {
extern crate my_crate;

#[test]
#[should_panic]
#[ignore]
fn test_hello2() {
    assert_eq!("cat hello--", my_crate::talk::cat::hello());
}

}