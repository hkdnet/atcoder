#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, X: u64, Y: u64);
    input!(s: [(u64, u64); N - 1]);
    input!(Q: usize);
    input!(q: [u64; Q]);

    let mut memo = vec![vec![0; N]; 840];

    let mut solve = |start_at: u64| -> u64 {
        let s1 = start_at + X;
        let mut t = s1;
        let mut non_memos = vec![];
        for i in 1..N {
            // 今バス停 i にいる
            let i = i - 1;
            let tt = t % 840;
            if memo[tt as usize][i] != 0 {
                t += memo[tt as usize][i];
                break;
            }
            non_memos.push(t);
            let (p, delta) = s[i];
            if t % p != 0 {
                t += p - (t % p);
            }
            t += delta;
        }

        for (idx, ti) in non_memos.into_iter().enumerate() {
            let m = ti % 840;
            memo[m as usize][idx] = t - ti;
        }

        t + Y
    };

    for qq in q {
        let ans = solve(qq);
        println!("{}", ans);
    }
}
