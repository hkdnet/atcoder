#![allow(unused_imports)]
use proconio::input;

const MOD: u64 = 1000000007;

fn main() {
    input!(s: usize);
    let mut dp = vec![0; s + 1];
    dp[0] = 1;

    for idx in 3..s + 1 {
        dp[idx] = dp[idx - 1] + dp[idx - 3];
        dp[idx] %= MOD;
    }

    println!("{}", dp[s]);
}

// 3 -> 1
// 4 -> 1
// 5 -> 1
// 6 -> 2
// 1 6
// 1 3, 3
// 7 -> 3
// 2 3, 4 | 4, 3
// 1 7
// 8 -> 4
// 2 3, 5 | 5, 3
// 1 4, 4
// 1 8
// 9 -> 4
// 3 3, 6 | 6, 3 | 3, 3, 3
// 1 4, 5
