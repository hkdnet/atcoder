fn main() {
    use proconio::input;
    input!(n: usize, d: i64);
    input!(xys: [(i64, i64); n]);

    let dd = d * d;
    let mut ans = 0;

    for (x, y) in xys {
        let dist = x * x + y * y;
        if dist <= dd {
            ans += 1;
        }
    }

    println!("{}", ans);
}
