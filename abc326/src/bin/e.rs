#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    let MOD = 998244353i64;
    input!(N: usize);
    let ni64 = N as i64;
    input!(a: [i64; N]);
    let mut dp = vec![0i64; N + 1];
    let mut memo = vec![0; N + 1];
    let inv_n = modinv(ni64, MOD);
    dp[N] = a[N - 1];
    memo[N] = dp[N];
    for i in 0..N {
        let i = N - i - 1; // N-1 -> 0
        if i != 0 {
            dp[i] = a[i - 1];
        }
        dp[i] += memo[i + 1] * inv_n;
        dp[i] %= MOD;
        memo[i] = (memo[i + 1] + dp[i]) % MOD;
    }
    println!("{}", dp[0]);
}

fn modinv(aa: i64, m: i64) -> i64 {
    let mut a = aa;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);

        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u
}
