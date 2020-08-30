#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(n: u128, a: u128, b: u128);
    let ab = a + b;
    let mul = n / ab;
    let ans = a * mul + std::cmp::min(a, n % ab);
    println!("{}", ans);
}
