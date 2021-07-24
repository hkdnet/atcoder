#![allow(unused_imports)]
use std::ops::Sub;

use proconio::input;
use proconio::marker::*;

use comp::union_find::UnionFind;

fn main() {
    input!(h: usize, w: usize, q_cnt: usize);
    let mut uf = UnionFind::new(h * w);
    let mut flags = vec![false; h * w];
    let index_of = |x: usize, y: usize| x * w + y;
    let neighbors = |x: usize, y: usize| -> Vec<usize> {
        let mut v = vec![];
        if x != 0 {
            v.push(index_of(x - 1, y));
        }
        if x != h - 1 {
            v.push(index_of(x + 1, y));
        }
        if y != 0 {
            v.push(index_of(x, y - 1));
        }
        if y != w - 1 {
            v.push(index_of(x, y + 1));
        }

        v
    };
    for _ in 0..q_cnt {
        input!(ty: Usize1);
        match ty {
            0 => {
                input!(x: Usize1, y: Usize1);
                let idx = index_of(x, y);
                flags[idx] = true;
                for n_idx in neighbors(x, y) {
                    if flags[n_idx] {
                        uf.unite(idx, n_idx);
                    }
                }
            }
            1 => {
                input!(ax: Usize1, ay: Usize1, bx: Usize1, by: Usize1);
                let a_index = index_of(ax, ay);
                let b_index = index_of(bx, by);
                let ans = if flags[a_index]
                    && flags[b_index]
                    && uf.same(index_of(ax, ay), index_of(bx, by))
                {
                    "Yes"
                } else {
                    "No"
                };
                println!("{}", ans);
            }
            _ => unreachable!("Unknown query type"),
        }
    }
}
