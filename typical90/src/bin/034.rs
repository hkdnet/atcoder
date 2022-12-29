#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, k: usize);
    input!(aa: [u32; n]);

    let mut s = std::collections::HashMap::new();
    // [l, r)
    let mut l = 0;
    let mut r = 0;
    let mut max_range = 0;
    while r < n {
        // check the next element
        if s.len() <= k {
            let v = aa[r];
            s.insert(v, r);
            r += 1;
            if s.len() <= k {
                max_range = std::cmp::max(max_range, r - l);
            }
            continue;
        }

        while l <= r {
            let v = aa[l];
            match s.get(&v) {
                Some(&idx) if idx == l => {
                    s.remove(&v);
                    l += 1;
                    break;
                }
                Some(_) | None => {
                    l += 1;
                }
            }
        }
    }
    println!("{}", max_range);
}
