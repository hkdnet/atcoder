#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
const MOD: u64 = 1000000007;
fn main() {
    input!(n: usize, aa: [u64; n]);

    let mut ss = vec![0; n + 1];
    for i in 0..n {
        let idx = n - i;
        ss[idx - 1] = ss[idx] + aa[idx - 1];
        ss[idx - 1] %= MOD;
    }
    let mut sum = 0;
    for i in 0..n {
        let a = aa[i];
        let s = ss[i + 1];
        let tmp = (s * a) % MOD;
        sum += tmp;
        sum %= MOD;
    }
    println!("{}", sum);
}
