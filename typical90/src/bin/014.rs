#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, aa: [i64; n], bb: [i64; n]);
    let mut aa = aa;
    let mut bb = bb;
    aa.sort_unstable();
    bb.sort_unstable();
    let ans: i64 = aa
        .into_iter()
        .zip(bb.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("{}", ans)
}
