static COLLATZ_NUMBER: u32 = 27;

fn main() {
    calc_collatz(COLLATZ_NUMBER, 0);
}

fn calc_collatz(n: u32, step: u32) {
    if step > 0 {
        println!("{} -> {}", step, n);
    }
    if n > 1 {
        calc_collatz(if n % 2 == 0 { n / 2 } else { 3 * n + 1 }, step + 1);
    }
}
