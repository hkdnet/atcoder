#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(a: u32, b: u32);
    for n in 0..10000 {
        let aa = (n as f64 * 0.08f64).floor() as u32;
        let bb = (n as f64 * 0.10f64).floor() as u32;
        if aa == a && bb == b {
            println!("{}", n);
            return;
        }
    }
    println!("{}", -1);
}
