#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(mut x: u32);
    let mut ans = 0;
    ans += (x / 500) * 1000;
    x %= 500;
    ans += (x / 5) * 5;

    println!("{}", ans);
}
