use proconio::input;

fn main() {
    input!(s: String);
    let len = s.len();
    let xxx = "x".repeat(len);
    println!("{}", xxx);
}
