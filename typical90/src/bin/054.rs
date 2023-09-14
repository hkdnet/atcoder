#![allow(unused_imports)]
use std::collections::BTreeSet;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, M: usize);
    let mut G = vec![BTreeSet::new(); N + M];
    for i in 0..M {
        input!(K: usize);
        input!(p: [Usize1; K]);
        let paper_node = N + i;
        for r in p {
            G[r].insert(paper_node);
            G[paper_node].insert(r);
        }
    }

    let mut t = vec![-2; N + M];
    t[0] = 0;
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(node) = q.pop_front() {
        for &next in G[node].iter() {
            if t[next] == -2 {
                t[next] = t[node] + 1;
                q.push_back(next);
            }
        }
    }

    for i in 0..N {
        println!("{}", t[i] / 2);
    }
}
