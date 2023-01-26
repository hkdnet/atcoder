#![allow(unused_imports)]
use std::collections::HashMap;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    let dx = [!0, 0, 1, 0];
    let dy = [0, 1, 0, !0];

    input!(h: usize, w: usize);
    input!(s: (Usize1, Usize1));
    input!(g: (Usize1, Usize1));
    input!(m: [Chars; h]);

    let mut q = VecDeque::new();
    q.push_front((0, s, 4));

    let step =
        |p: (usize, usize), dir: usize| (p.0.wrapping_add(dx[dir]), p.1.wrapping_add(dy[dir]));

    let mut dist = vec![vec![std::i32::MAX; w]; h];

    while let Some((c, p, d)) = q.pop_front() {
        if dist[p.0][p.1] < c {
            continue;
        }
        dist[p.0][p.1] = c;

        for dd in 0..4 {
            let nx = step(p, dd);
            if nx.0 < h && nx.1 < w && m[nx.0][nx.1] == '.' {
                if d == dd {
                    q.push_front((c, nx, dd));
                } else {
                    q.push_back((c + 1, nx, dd));
                }
            }
        }
    }

    println!("{}", dist[g.0][g.1] - 1);
}
