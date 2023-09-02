#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: i32, M: i32, P: i32);
    let rest = N - M;
    if rest < 0 {
        println!("0");
    } else {
        println!("{}", (rest / P) + 1);
    }
}
