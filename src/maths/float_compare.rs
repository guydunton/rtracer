pub fn is_same(a: f64, b: f64) -> bool {
    use std::f64;
    let result = a - b;
    result.abs() <= (f64::EPSILON * 100.0)
}

#[test]
fn test_is_same() {
    assert_eq!(is_same(0.0, 0.0), true);
    assert_eq!(is_same(1.0, 1.0), true);
    assert_eq!(is_same(1.0, 0.0), false);
    assert_eq!(is_same(1.0, 1.1), false);
}
