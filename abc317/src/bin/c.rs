#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);
    let edges = {
        let mut v = vec![vec![u64::MAX; N]; N];
        for _ in 0..M {
            input!(a: Usize1, b: Usize1, c: u64);
            v[a][b] = c;
            v[b][a] = c;
        }
        v
    };
    let mut ans = 0;
    for v in (0..N).permutations(N) {
        let mut from = v[0];
        let mut tmp = 0;
        for &to in &v[1..] {
            if edges[from][to] == u64::MAX {
                break;
            }
            tmp += edges[from][to];
            from = to;
        }
        ans = std::cmp::max(ans, tmp);
    }
    println!("{}", ans);
}
