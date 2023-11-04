#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;
use proconio::marker::*;
fn main() {
    input!(N: usize);
    input!(ps: [f64; N]);

    let mut dp = vec![-1200f64; N + 1];
    dp[0] = 0.;

    for new_val in ps.iter() {
        for j in 0..N {
            let j = N - j - 1;
            if dp[j] == -1200f64 {
                continue;
            }
            // pick
            let nu = dp[j] * 0.9 + new_val;
            if dp[j + 1] < nu {
                dp[j + 1] = nu;
            }
        }
    }

    let mut ans = -1200f64;

    let mut d = 1f64;
    let mut dd = 1f64;
    for (k, u) in dp.into_iter().enumerate() {
        if k == 0 {
            continue;
        }
        let score = u / d;
        let score = score - 1200. / (k as f64).sqrt();

        if ans < score {
            ans = score;
        }

        dd *= 0.9;
        d += dd;
    }
    println!("{:.8}", ans);
}
