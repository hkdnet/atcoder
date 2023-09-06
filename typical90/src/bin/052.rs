#![allow(unused_imports)]
use std::mem::swap;

use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    let MOD = 1_000_000_007i64;
    input!(N: usize);
    let mut d = Vec::with_capacity(N);

    let mut ans = 1;
    for _ in 0..N {
        input!(a: [i64; 6]);
        let mut sum = 0;
        for v in a.iter() {
            sum += v;
        }
        ans *= sum;
        ans %= MOD;
        d.push(a);
    }

    println!("{}", ans);
}
