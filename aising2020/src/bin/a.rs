fn main() {
    use proconio::input;
    input!(l: u32, r: u32, d: u32);
    let mut ans = 0;
    for n in l..r + 1 {
        if n % d == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
