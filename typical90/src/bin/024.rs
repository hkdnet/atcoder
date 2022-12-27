#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, k: i64);
    input!(aa: [i64; n]);
    input!(bb: [i64; n]);

    let diffs = aa.iter().zip(bb.iter()).map(|(&a, &b)| (a - b).abs());
    let mut sum = 0;
    for d in diffs {
        sum += d;
    }
    if k >= sum && k % 2 == sum % 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
