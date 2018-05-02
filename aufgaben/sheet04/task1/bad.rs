/// print all happy primes between 1 and 20
fn main() {
    for i in 1..21 {
        if is_happy(i) && is_prime(i) {
            println!("{} is a happy prime!", i);
        }
    }
}

/// check if given i32 is a [happy number][1]?
///
/// [1]: https://en.wikipedia.org/wiki/Happy_number
fn is_happy(n: i32) -> bool {
    fn to_digits(mut n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        while n > 0 {
            res.push(n % 10);
            n = n / 10;
        }
        res
    }
    match n {
        1 => true,
        4 => false,
        t => is_happy(to_digits(t).iter().map(|v| v*v).sum()),
    }
}

/// check if given i32 is priem?
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for d in 2..n {
        if n % d == 0 {
            return false;
        }
    }
    true
}
