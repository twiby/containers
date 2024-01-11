use containers;

#[test]
fn dummy() {
    let ret = containers::dummy();
    assert_eq!(&ret, "hello world");
}
