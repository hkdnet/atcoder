#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, k: usize, cs: Chars);
    let indexer = |c: char| -> usize {
        let idx = c.to_digit(36).unwrap();
        (idx - 10) as usize
    };
    let mut ans = vec![];
    let mut l = 0;
    while ans.len() != k {
        let mut tmp = [None; 26];
        let r = n + ans.len() - k;
        let mut c_idx = l;
        // println!("l = {}, r = {}", l, r); // debug
        while c_idx <= r {
            let idx = indexer(cs[c_idx]);
            if tmp[idx] == None {
                tmp[idx] = Some(c_idx);
            }
            c_idx += 1;
        }
        // println!("{:?}", tmp); // debug
        for &e in tmp.iter() {
            if let Some(idx) = e {
                ans.push(cs[idx]);
                l = idx + 1;
                break;
            }
        }
    }
    println!(
        "{}",
        ans.into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    )
}
