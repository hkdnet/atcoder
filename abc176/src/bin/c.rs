fn main() {
    use proconio::input;
    input!(n: u32);
    input!(aa: [u64; n]);

    let mut max = aa[0];
    let mut ans = 0;
    for a in aa {
        if a < max {
            ans += max - a;
        } else if max < a {
            max = a;
        }
    }

    println!("{}", ans);
}
