#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(d: usize, t: usize, s: usize);

    let m = if d % s == 0 { d / s } else { d / s + 1 };
    if m <= t {
        println!("Yes");
    } else {
        println!("No");
    }
}
