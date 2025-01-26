#[test]
fn test_embedded_test() {
    assert_snapshot!("Just a string", @"Just a string");
}
