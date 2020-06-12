fn main() {
    use proconio::input;

    input!(s: String, t: String);
    input!(a: usize, b: usize);
    input!(u: String);

    if u == s {
        println!("{} {}", a - 1, b);
    } else {
        println!("{} {}", a, b - 1);
    }
}
