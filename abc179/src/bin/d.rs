#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

const MOD: u32 = 998244353;

fn main() {
    input!(n: usize, k: usize);
    let mut v = vec![];
    for _ in 0..k {
        input!(l: usize, r: usize);
        let d = r - l;
        v.push((l, d));
    }

    use std::collections::BTreeMap;
    let mut dp: Vec<BTreeMap<usize, u32>> = vec![BTreeMap::new(); n + 1];
    dp[1].insert(0, 1);

    for i in 1..n {
        if dp[i].len() == 0 {
            continue;
        }
        for &(l, d) in v.iter() {
            let idx = i + l;
            if idx > n {
                break;
            }
            let mut m = dp[idx].clone();
            for (k, &v) in dp[i].iter() {
                let kk = k + d;
                match dp[idx].get(&kk) {
                    Some(a) => {
                        m.insert(kk, v + a);
                    }
                    None => {
                        m.insert(kk, v);
                    }
                }
            }

            dp[idx] = m;
        }
    }

    println!("{:?}", dp);
    let mut cnt = 0;
    for i in 0..n + 1 {
        let m = &dp[i];
        if m.len() == 0 {
            continue;
        }
        for (k, v) in m.iter() {
            if i + k >= n {
                cnt += v;
                cnt %= MOD;
            }
        }
    }
    println!("{}", cnt);
}
