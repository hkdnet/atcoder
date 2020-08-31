#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeSet;

fn osak(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }
    let mut ret = vec![0; n + 1];
    ret[0] = 0;
    ret[1] = 1;

    for i in 2..n + 1 {
        if ret[i] == 0 {
            let mut idx = i;
            while idx < n + 1 {
                ret[idx] = i;
                idx += i;
            }
        }
    }

    ret
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn setgcd(arr: &Vec<usize>) -> usize {
    let n = arr.len();
    if n == 1 {
        return arr[0];
    }
    let l = n / 2;
    let mut v = vec![0; if n % 2 == 0 { l } else { l + 1 }];
    for i in 0..l {
        let idx = 2 * i;
        let g = gcd(arr[idx], arr[idx + 1]);
        v[i] = g;
    }
    if n % 2 != 0 {
        v[l] = arr[n - 1];
    }
    setgcd(&v)
}
fn main() {
    use proconio::input;
    input!(n: usize);
    input!(aa: [usize; n]);

    let max = 1000000;
    if setgcd(&aa) == 1 {
        let osak = osak(max);
        let mut used = BTreeSet::new();
        for &a in aa.iter() {
            if a == 1 {
                continue;
            }
            let mut tmp = a;
            let mut v = vec![];
            while tmp != osak[tmp] {
                let p = osak[tmp];
                if used.contains(&p) {
                    println!("setwise coprime");
                    return;
                }
                v.push(p);
                tmp /= p;
            }
            if used.contains(&tmp) {
                println!("setwise coprime");
                return;
            }

            for e in v {
                used.insert(e);
            }
            used.insert(tmp);
        }
        println!("pairwise coprime");
    } else {
        println!("not coprime");
    }
}
