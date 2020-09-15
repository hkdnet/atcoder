#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(a: i64, b: i64, c: i64, d: i64);
    let mut ans = a * c;
    for v in vec![a*d, b*c, b*d] {
        ans = std::cmp::max(ans, v);
    }

    if ans < 0 {
        if a *b < 0 || c * d < 0 {
            ans = 0;
        }
    }

    println!("{}", ans);
}
