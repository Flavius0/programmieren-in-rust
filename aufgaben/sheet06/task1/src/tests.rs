use {clamp, sum_prod, ToOptionExt};

#[test]
fn clamp_val() {
    assert_eq!(clamp(5, 0, 9), 5);
    assert_eq!(clamp(5, 5, 9), 5);
    assert_eq!(clamp(5, 0, 5), 5);
    assert_eq!(clamp(5, 5, 5), 5);
}

#[test]
fn clamp_min() {
    assert_eq!(clamp(-5, 0, 9), 0);
    assert_eq!(clamp(-5, 5, 9), 5);
    assert_eq!(clamp(-5, 0, 5), 0);
    assert_eq!(clamp(-5, 5, 5), 5);
}

#[test]
fn clamp_max() {
    assert_eq!(clamp(50, 0, 9), 9);
    assert_eq!(clamp(50, 5, 9), 9);
    assert_eq!(clamp(50, 0, 5), 5);
    assert_eq!(clamp(50, 5, 5), 5);
}

#[test]
fn test_sum_prod() {
    assert_eq!(sum_prod(4,3), (7, 12));
}

#[test]
fn test_to_option() {
    assert_eq!(true.into_option(3), Some(3));
    assert_eq!(false.into_option(3), None);
}
