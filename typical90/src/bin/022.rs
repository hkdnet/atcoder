#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(a: u64, b: u64, c: u64);
    let x = gcd(gcd(a, b), c);

    println!("{}", (a + b + c) / x - 3);
}

fn gcd<T: PartialEq + PartialOrd + From<u8> + std::ops::Rem<Output = T> + Copy>(a: T, b: T) -> T {
    if b < a {
        return gcd(b, a);
    }

    let mut x = a;
    let mut y = b;
    let mut r = x % y;
    while r != T::from(0u8) {
        x = y;
        y = r;
        r = x % y;
    }

    y
}
