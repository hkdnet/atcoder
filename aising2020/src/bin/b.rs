fn main() {
    use proconio::input;
    input!(n: usize);
    input!(aa: [u32; n]);

    let mut ans = 0;
    for i in 0..n {
        if i % 2 == 0 {
            if aa[i] % 2 == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
