fn count(s: &str, c: char) -> u32 {
    let mut res = 0;
    for t in s.chars() {
        if c == t {
            res = res + 1;
        }
    }
    res
}

#[test]
fn test_length() {
    assert_eq!(count("peter", 'e'), 2);
    assert_eq!(count("peter", 'p'), 1);
    assert_eq!(count("peter", 'x'), 0);
}
