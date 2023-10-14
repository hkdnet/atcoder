#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

fn vec_eq(v1: &Vec<usize>, v2: &Vec<usize>) -> bool {
    if v1[0] < v2[0] {
        return false;
    }
    for i in 1..10 {
        if v1[i] != v2[i] {
            return false;
        }
    }

    true
}
fn d2v(n: usize) -> Vec<usize> {
    let mut v = vec![0; 10];
    let mut n = n;
    v[n % 10] += 1;
    while n >= 10 {
        n /= 10;
        v[n % 10] += 1;
    }

    v
}
fn s2v(s: &String) -> Vec<usize> {
    let mut v = vec![0; 10];
    for c in s.chars() {
        let d = c.to_digit(10).unwrap() as usize;
        v[d] += 1;
    }

    v
}
fn main() {
    input!(N: usize);
    input!(S: String);

    let MAX = {
        let mut tmp = 1usize;
        for _ in 0..S.len() {
            tmp *= 10;
        }
        tmp
    };

    let s = s2v(&S);

    let mut ans = 0;
    for i in 0..=MAX {
        let n = i * i;
        if n >= MAX {
            break;
        }
        let v2 = d2v(n);
        if vec_eq(&s, &v2) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
