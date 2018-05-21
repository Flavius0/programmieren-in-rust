use Vector2;

#[test]
fn create_vector2() {
    assert_eq!(Vector2::new(0, 0), Vector2::origin());
    assert_eq!(Vector2::new(1.0, 0.0), Vector2::unit_x());
    assert_eq!(Vector2::new(0, 1), Vector2::unit_y());
    assert_ne!(Vector2::<i32>::unit_x(), Vector2::unit_y());
}
