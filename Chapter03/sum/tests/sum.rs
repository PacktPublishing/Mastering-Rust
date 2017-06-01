extern crate sum;

#[test]
fn test_sum_integration() {
    assert_eq!(sum::sum(6, 8), 14);
}
