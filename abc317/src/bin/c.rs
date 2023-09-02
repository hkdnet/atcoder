#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize, edges: (Usize1, Usize1, u64));

    let E = {
      let mut e = vec![vec![0; N];N];

      for (a, b, c) in edges {
        e[a][b] = c;
        e[b][a] = c;
      }

      e
    };

    let mut ans = 0;
    let mut used = vec![false; N];
    let dfs = |cur, d| {

    };

}
