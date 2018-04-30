fn main() {
    let a = 5;
    test_number(a);

    let (mut x, z) = (3, 4);

    let mut y = &x;
    // *y = 15;
    y = &z;
}

fn test_number(a: i32) {
    let b = match a {
        1 | 2 => "one or two",
        3 ... 10 => "between three and ten",
        n if n > 10 => "greater than ten",
        _ => "somthing else",
    };
    println!("{}", b);
}
