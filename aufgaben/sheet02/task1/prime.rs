//! Aufgabe 2.1: Primzahltest

fn main() {
    for i in 1..21 {
        println!("{}{}", i, if is_prime(i) { "*" } else { "" })
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    } else if n == 2 {
        return true;
    }

    let f = n as f32;
    let bound = f.sqrt().ceil() as u32;

    for i in 2..bound + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
