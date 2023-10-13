#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    let MOD = 998244353;
    input!(N: usize, X: usize);
    input!(ts: [usize; N]);
    let len = X + 1;
    let ni64 = N as i64;
    // stopped, playing 1, playing others
    let mut dp = vec![vec![0i64; len], vec![0; len]];

    let n_inv = modinv(ni64, MOD);
    dp[0][0] = 1;
    for i in 0..X {
        if dp[0][i] == 0 {
            continue;
        }
        let mi = dp[0][i] * n_inv % MOD;
        for (t_idx, &t) in ts.iter().enumerate() {
            if t_idx == 0 {
                for d in 1..t {
                    let idx = i + d;
                    if idx < len {
                        dp[1][idx] += mi;
                        dp[1][idx] %= MOD;
                    } else {
                        break;
                    }
                }
            }

            if i + t < len {
                dp[0][i + t] += mi;
                dp[0][i + t] %= MOD;
            }
        }
    }

    let mut ans = dp[1][X];
    ans += dp[0][X] * n_inv % MOD;
    ans %= MOD;
    println!("{}", ans);
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
