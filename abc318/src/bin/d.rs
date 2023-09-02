#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize);
    let mut e = std::collections::BTreeMap::new();
    let mut dp = vec![0; 131072];

    for i in 0..N - 1 {
        let i2 = 2 << i;
        for j in i + 1..N {
            let j2 = 2 << j;
            input!(d: i64);

            e.insert(i2 | j2, d);
            dp[i2 | j2] = d;
        }
    }
    let mut q = std::collections::VecDeque::new();
    for &k in e.keys() {
        q.push_back(k);
    }

    while let Some(s) = q.pop_front() {
        for (&k, &v) in e.iter() {
            if s & k == 0 {
                let nx = s | k;
                let cur = dp[s];
                let nv = cur + v;
                if dp[nx] < nv {
                    dp[nx] = nv;
                    q.push_back(nx);
                }
            }
        }
    }

    let mut ans = 0;
    for a in dp {
        if a > ans {
            ans = a;
        }
    }
    println!("{}", ans);
}
