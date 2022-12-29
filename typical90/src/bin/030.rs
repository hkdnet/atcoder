#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input!(n: usize, k: i32);
    let mut c = vec![0; n + 1];

    let mut osak = vec![0; n + 1];
    osak[1] = 1;
    for i in 2..=n {
        if osak[i] != 0 {
            continue;
        }
        let mut idx = i;
        while idx < osak.len() {
            osak[idx] = idx / i;
            idx += i;
        }
    }

    for i in 2..=n {
        if osak[i] == 1 {
            let mut idx = i;
            while idx < c.len() {
                c[idx] += 1;
                idx += i;
            }
        }
    }

    let ans = c.into_iter().filter(|&v| v >= k).count();
    println!("{}", ans)
}
