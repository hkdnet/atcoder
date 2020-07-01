fn main() {
    use proconio::input;
    input!(n: u32);
    let n = n % 1000;
    if n == 0 {
        println!("{}", 0);
    } else {
        println!("{}", 1000 - n);
    }
}
