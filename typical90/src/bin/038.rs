#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(a: u128, b: u128);
    let g = gcd(a, b);
    let ans = a / g * b;
    if ans > 1_000_000_000_000_000_000 {
        println!("Large")
    } else {
        println!("{}", ans);
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
