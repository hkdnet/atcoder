#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeSet;

fn f(n: usize) -> Vec<String> {
    let mut tmp = vec![("".to_string(), 0)];
    for _ in 0..n {
        let t = tmp.iter().flat_map(|(s, cnt)| {
            let mut v = vec![(format!("{}(", s), cnt + 1)];
            if *cnt != 0 {
                v.push((format!("{})", s), cnt - 1));
            }
            v
        });
        tmp = t.collect();
    }
    tmp.into_iter()
        .filter_map(|(s, c)| if c == 0 { Some(s) } else { None })
        .collect()
}
fn main() {
    input!(n: usize);
    if n % 2 != 0 {
        return;
    }

    for s in f(n) {
        println!("{}", s)
    }
}
