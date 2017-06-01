#[test]
#[should_panic]
fn test_panic() {
    panic!("Succeeded in failing!");
}
