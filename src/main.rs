fn main() {
    // println!("Hello, world!");
    println!("{}", factorial(20));
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}