#![allow(unused_imports)]
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

const MAX: u64 = std::u64::MAX;

fn main() {
    // 最小カットに帰着させる。

    input!(n: usize, w: u64);
    let mut d = Dinic::new(n + 2);

    // 0 + h1 + n+1
    //   |    |
    //   + h2 +
    let mut sum = 0;
    input!(aa: [u64; n]);
    for (i, a) in aa.into_iter().enumerate() {
        d.add(0, i + 1, a);
        sum += a;
        d.add(i + 1, n + 1, w);
    }

    for i in 0..n {
        input!(k: usize);
        input!(keys: [usize; k]);
        for k in keys {
            d.add(k, i + 1, MAX);
        }
    }

    let mf = d.max_flow(0, n + 1);
    println!("{}", sum - mf);
}

// 燃やす埋める
pub struct Dinic {
    n: usize,
    v: Vec<Vec<DinicEdge>>,
    level: Vec<Option<u64>>,
    iter: Vec<usize>,
}
impl Dinic {
    pub fn new(n: usize) -> Self {
        Dinic {
            n,
            v: vec![vec![]; n],
            level: vec![None; n],
            iter: vec![0; n],
        }
    }
    pub fn add(&mut self, from: usize, to: usize, cap: u64) {
        let cur_from_len = self.v[from].len();
        let cur_to_len = self.v[to].len();

        self.v[from].push(DinicEdge {
            to,
            rev: cur_to_len,
            capacity: cap,
        });
        self.v[to].push(DinicEdge {
            to: from,
            rev: cur_from_len,
            capacity: 0,
        });
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level = vec![None; self.n];
        self.level[s] = Some(0);
        let mut q = VecDeque::new();
        q.push_front(s);
        while let Some(node) = q.pop_front() {
            for i in self.v[node].iter() {
                if i.capacity > 0 && self.level[i.to].is_none() {
                    self.level[i.to] = Some(self.level[node].unwrap() + 1);
                    q.push_back(i.to);
                }
            }
        }
        return !self.level[t].is_none();
    }

    fn dfs(&mut self, node: usize, t: usize, f: u64) -> u64 {
        if node == t {
            return f;
        };
        for i in self.iter[node]..self.v[node].len() {
            let cur_capacity = self.v[node][i].capacity;
            let cur_to = self.v[node][i].to;
            let cur_rev = self.v[node][i].rev;

            if cur_capacity > 0 && self.level[node] < self.level[cur_to] {
                let d = self.dfs(cur_to, t, std::cmp::min(f, cur_capacity));
                if d > 0 {
                    self.v[node][i].capacity -= d;
                    self.v[cur_to][cur_rev].capacity += d;
                    return d;
                }
            }
        }
        return 0;
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> u64 {
        let mut res = 0;
        loop {
            self.bfs(s, t);
            if self.level[t].is_none() {
                return res;
            }
            self.iter = vec![0; self.n];
            loop {
                let f = self.dfs(s, t, MAX);
                if f > 0 {
                    res += f;
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct DinicEdge {
    to: usize,
    rev: usize,
    capacity: u64,
}
