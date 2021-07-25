#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

fn main() {
    input!(n: usize, m: usize, ps: [(Usize1, Usize1, usize); m]);
    let mut paths: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    for (a, b, cost) in ps {
        paths
            .entry(a)
            .and_modify(|e| {
                e.insert(b, cost);
            })
            .or_insert_with(|| {
                let mut m = BTreeMap::new();
                m.insert(b, cost);
                m
            });
        paths
            .entry(b)
            .and_modify(|e| {
                e.insert(a, cost);
            })
            .or_insert_with(|| {
                let mut m = BTreeMap::new();
                m.insert(a, cost);
                m
            });
    }
    // dijkstra
    let builder = |start: usize| -> Vec<usize> {
        let mut ret = vec![None; n];
        let mut q = BinaryHeap::new();
        ret[start] = Some(0);
        q.push(Reverse((0, start)));
        while let Some(Reverse((cur_cost, a))) = q.pop() {
            if let Some(c) = ret[a] {
                if c > cur_cost {
                    continue;
                }
            }
            if let Some(m) = paths.get(&a) {
                for (&b, &cost) in m.iter() {
                    let new_cost = cur_cost + cost;
                    ret[b] = ret[b]
                        .map(|c| {
                            if new_cost < c {
                                q.push(Reverse((new_cost, b)));
                                new_cost
                            } else {
                                c
                            }
                        })
                        .or_else(|| {
                            q.push(Reverse((new_cost, b)));
                            Some(new_cost)
                        });
                }
            }
        }

        ret.into_iter().map(|e| e.unwrap()).collect()
    };

    let head = builder(0);
    let tail = builder(n - 1);

    for via in 0..n {
        let cost = head[via] + tail[via];
        println!("{}", cost);
    }
}
