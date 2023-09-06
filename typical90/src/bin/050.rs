#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    let MOD = 1_000_000_007;
    input!(N: usize, L: usize);
    let mut v = vec![0; 200001];
    v[0] = 1;
    for i in 0..=N {
        v[i + 1] += v[i];
        v[i + 1] %= MOD;
        v[i + L] += v[i];
        v[i + L] %= MOD;
    }

    println!("{}", v[N]);
}
