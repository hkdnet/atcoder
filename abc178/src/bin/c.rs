#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

const MOD:u64 =  1000000007 ;
fn calc(base: u64,pow:u64 ) -> u64{
    if pow == 0 {
        return 1;
    }
    if pow == 1 {
        return base;
    }
    let m = pow / 2;
    let t = calc(base, m);
    let mut ret = ( t * t) % MOD;
    if pow % 2 != 0 {
        ret = ( ret * base) % MOD;
    }
    ret
}

fn main() {
    input!(n: u64);

    // 全体 - 0 なし - 9 なし + (0, 9)なし
    let mut ans = calc(10, n);
    ans += calc(8, n);
    ans %= MOD;
    let tmp = calc(9, n);
    ans += MOD - tmp;
    ans %= MOD;
    ans += MOD - tmp;
    ans %= MOD;

    println!("{}", ans);
}
