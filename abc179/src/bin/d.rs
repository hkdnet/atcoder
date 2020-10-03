#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

const MOD: u64 = 998244353;

fn main() {
    input!(n: usize, k: usize);
    let allowed: Vec<(usize, usize)> = {
        let mut tmp = std::collections::BTreeSet::new();
        for _ in 0..k {
            input!(l: usize, r: usize);
            tmp.insert((l, r));
        }
        tmp.into_iter().collect()
    };

    let mut dp = vec![0; n];
    dp[0] = 1;
    dp[1] = MOD - 1;
    for i in 0..n {
        // println!("i = {}", i);
        if i != 0 {
            dp[i] += dp[i - 1];
            dp[i] %= MOD;
        }
        for &(l, r) in allowed.iter() {
            if i + l < n {
                dp[i + l] += dp[i];
                dp[i + l] %= MOD;
            }
            if i + r + 1 < n {
                dp[i + r + 1] += MOD - dp[i];
                dp[i + r + 1] %= MOD;
            }
            // println!("{:?}", dp);
        }
    }

    println!("{}", dp[n - 1]);
}
