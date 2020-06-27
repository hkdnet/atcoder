fn main() {
    use proconio::input;
    input!(a: i32);
    let ans = a + a * a + a * a * a;
    println!("{}", ans);
}
