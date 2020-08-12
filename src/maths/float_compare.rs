pub fn is_same(a: f64, b: f64) -> bool {
    use std::f64;
    let result = a - b;
    result.abs() <= (f64::EPSILON * 100.0)
}

#[allow(unused)]
pub fn round(v: f64) -> f64 {
    const SIG_FIGS: f64 = 100000.0;
    (v * SIG_FIGS).round() / SIG_FIGS
}

#[test]
fn test_is_same() {
    assert_eq!(is_same(0.0, 0.0), true);
    assert_eq!(is_same(1.0, 1.0), true);
    assert_eq!(is_same(1.0, 0.0), false);
    assert_eq!(is_same(1.0, 1.1), false);
}
