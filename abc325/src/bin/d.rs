#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize);
    input!(ts: [(usize, usize); N]);
    let v = {
        let mut v = ts.iter().map(|&(t, d)| (t, t + d)).collect_vec();
        v.sort_unstable();
        v
    };

    let mut ans = 0;
    let mut v_idx = 0;
    let mut t = 0;
    let mut q = BinaryHeap::new();
    while v_idx < v.len() || !q.is_empty() {
        while v_idx < v.len() {
            let (l, r) = v[v_idx];
            if t < l {
                if q.is_empty() {
                    t = l;
                } else {
                    break;
                }
            }
            if t != l {
                break;
            }
            q.push(Reverse(r));
            v_idx += 1;
        }

        while let Some(Reverse(r)) = q.peek() {
            // dbg!(t, r);
            if *r < t {
                q.pop();
            } else {
                break;
            }
        }
        if !q.is_empty() {
            q.pop();
            // println!("print at {}", t);
            ans += 1;
            t += 1;
        }
    }

    println!("{}", ans);
}
