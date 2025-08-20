use rust_dsa::problems::two_sum::two_sum;

#[test]
fn test_found() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn test_not_found() {
    assert_eq!(two_sum(vec![1, 2, 3], 7), None);
}
