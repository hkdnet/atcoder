#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, X: u64, Y: u64);
    input!(s: [(u64, u64); N - 1]);
    input!(Q: usize);
    input!(q: [u64; Q]);

    let mut memo = vec![0; 840];

    let mut solve = |start_at: u64| -> u64 {
        let s1 = start_at + X;
        let mut t = s1;
        let memo_key = (t % 840) as usize;
        if memo[memo_key] != 0 {
            t += memo[memo_key];
        } else {
            for i in 1..N {
                // 今バス停 i にいる
                let i = i - 1;
                let (p, delta) = s[i];
                if t % p != 0 {
                    t += p - (t % p);
                }
                t += delta;
            }
            memo[memo_key] = t - s1;
        }
        t + Y
    };

    for qq in q {
        let ans = solve(qq);
        println!("{}", ans);
    }
}
