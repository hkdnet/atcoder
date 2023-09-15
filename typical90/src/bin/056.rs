#![allow(unused_imports)]
use std::collections::BTreeMap;
use std::mem::swap;

use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, S: usize);
    input!(ab: [(usize, usize); N]);

    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;
    for d in 1..=N {
        let (a, b) = ab[d - 1];
        for i in std::cmp::min(a, b)..=S {
            match (a <= i, b <= i) {
                (true, true) => dp[d][i] = dp[d - 1][i - a] || dp[d - 1][i - b],
                (true, false) => dp[d][i] = dp[d - 1][i - a],
                (false, true) => dp[d][i] = dp[d - 1][i - b],
                (false, false) => {
                    unreachable!()
                }
            }
        }
    }
    if dp[N][S] {
        let mut v = vec!['A'; N];
        let mut rest = S;
        for i in 0..N {
            let i = N - i - 1;
            let (a, b) = ab[i];
            if rest < a {
                // b
                v[i] = 'B';
                rest -= b;
            } else {
                if !dp[i][rest - a] {
                    v[i] = 'B';
                    rest -= b;
                } else {
                    rest -= a;
                }
            }
        }
        println!("{}", v.iter().collect::<String>());
    } else {
        println!("Impossible");
    }
}
