#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use comp::binary_search;
use std::collections::BTreeMap;

fn build_acc(nn: &Vec<i32>, p: i32) -> Vec<i32> {
    let n = nn.len();
    let mut tmp = vec![0; n + 1];
    let mut mul = 1;

    for i in 0..n {
        let i = n - i;
        tmp[i - 1] = tmp[i] + (nn[i - 1] * mul);
        tmp[i - 1] %= p;
        mul *= 10;
        mul %= p;
    }
    tmp
}

#[test]
fn test_build_acc() {
    let nn = vec![1, 2, 3];
    assert_eq!(build_acc(&nn, 3), vec![0, 2, 0, 0]);

    let nn = vec![4, 2, 3];
    assert_eq!(build_acc(&nn, 3), vec![0, 2, 0, 0]);
    let nn = vec![8, 5, 9];
    assert_eq!(build_acc(&nn, 2), vec![1, 1, 1, 0]);
    let nn = vec![9, 8, 9, 2];
    assert_eq!(build_acc(&nn, 2), vec![0, 1, 1, 0, 0]);
}

fn main() {
    input!(n: usize, p: i32, cc: Chars);
    let nn: Vec<i32> = cc
        .iter()
        .map(|c| c.to_digit(10).unwrap() as i32 % p)
        .collect();

    if p == 2 || p == 5 {
        let mut ans = 0;
        for i in 0..n {
            if nn[i] % p == 0 {
                ans += i + 1;
            }
        }
        println!("{}", ans);
        return;
    }

    let acc = build_acc(&nn, p);
    let mut counters: BTreeMap<i32, usize> = BTreeMap::new();
    for (_, &v) in acc.iter().enumerate() {
        *counters.entry(v).or_default() += 1;
    }
    let mut cnt = 0;
    for (_, &v) in counters.iter() {
        cnt += (v * (v - 1)) / 2;
    }

    println!("{}", cnt);
}
