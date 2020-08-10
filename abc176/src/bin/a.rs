fn main() {
    use proconio::input;
    input!(n: usize, x: usize, t: usize);
    let m = if n % x == 0 { n / x } else { n / x + 1 };

    println!("{}", m * t);
}
