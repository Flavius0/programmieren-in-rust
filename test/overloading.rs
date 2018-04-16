
fn hello() {
    println!("Hello World!");
}

fn hello(a: i32) {
    println!("Hello {}", a);
}

fn main() {
    hello();
    hello(100);
}
