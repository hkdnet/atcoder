#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;

fn main() {
    input!(n: usize, jobs: [(usize, usize, usize); n]);
    let mut jobs = jobs;
    jobs.sort_unstable();
    // println!("{:?}", jobs); // debug

    let mut dp = BTreeMap::new();
    {
        let (due, cost, score) = jobs[0];
        if cost <= due {
            dp.insert(cost, score);
        }
    }
    // println!("{:?}", dp); // debug
    for (due, cost, score) in jobs.into_iter().skip(1) {
        let mut nx = BTreeMap::new();
        for (&cur_cost, &cur_score) in dp.iter() {
            let key1 = cost + cur_cost;
            if key1 <= due {
                nx.entry(key1)
                    .and_modify(|e| {
                        *e = std::cmp::max(cur_score + score, *e);
                    })
                    .or_insert(cur_score + score);
            }
            nx.entry(cur_cost)
                .and_modify(|e| {
                    *e = std::cmp::max(cur_score, *e);
                })
                .or_insert(cur_score);
        }
        if cost <= due {
            nx.entry(cost)
                .and_modify(|e| {
                    *e = std::cmp::max(*e, score);
                })
                .or_insert(score);
        }
        std::mem::swap(&mut nx, &mut dp);
        // println!("{:?}", dp); // debug
    }
    let mut ans = 0;
    for &score in dp.values() {
        ans = std::cmp::max(ans, score);
    }
    println!("{}", ans)
}
