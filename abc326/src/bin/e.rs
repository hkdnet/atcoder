#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    let MOD = 998244353i64;
    input!(N: usize);
    input!(a: [i64; N]);
    let mut x = modinv(N as i64, MOD);
    let mut dp = vec![0; N + 1];
    let mut sum = 0;
    for i in a {
        sum += i * x % MOD;
        sum %= MOD;
        x = modinv(x, MOD);
    }
    println!("{}", sum);
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
