#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

const MOD: u64 = 1_000_000_007;

fn main() {
    input!(k: usize);
    if k == 0 || k % 9 != 0 {
        println!("{}", 0);
        return;
    }
    let mut dp = vec![0; k + 10];
    dp[0] = 1;

    for i in 0..k {
        for j in 1..=9 {
            dp[i + j] += dp[i];
            dp[i + j] %= MOD;
        }
    }
    println!("{}", dp[k]);
}
