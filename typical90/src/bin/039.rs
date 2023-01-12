#![allow(unused_imports)]
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize);
    let mut edges = vec![vec![]; n];
    for _ in 0..n - 1 {
        input!(a: Usize1, b: Usize1);
        edges[a].push(b);
        edges[b].push(a);
    }

    let t = T::new(edges);
    let mut ans = 0;
    let nu64 = n as u64;
    for i in 0..n {
        let k = t.dp[i];
        ans += k * (nu64 - k);
    }

    println!("{}", ans);
}
struct T {
    edges: Vec<Vec<usize>>,
    dp: Vec<u64>,
}
impl T {
    fn new(edges: Vec<Vec<usize>>) -> Self {
        let n = edges.len();
        let mut t = T {
            edges,
            dp: vec![0; n],
        };
        t.dfs();
        t
    }

    fn dfs(&mut self) {
        self.sub(0, self.edges.len() + 2);
    }

    fn sub(&mut self, i: usize, from: usize) {
        let children = self.edges[i].clone();
        let mut tmp = 1;
        for j in children {
            if j != from {
                self.sub(j, i);
                tmp += self.dp[j];
            }
        }
        self.dp[i] = tmp;
    }
}
