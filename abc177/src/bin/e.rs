#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeSet;

fn primes_upto(n: u32) -> BTreeSet<u32> {
    let nu = n as usize;
    let mut v = vec![true; nu + 1];

    for i in 2..(nu / 2) {
        if v[i] {
            let mut idx = i * 2;
            while idx <= nu {
                v[idx] = false;

                idx += i;
            }
        }
    }
    v[0] = false;
    v[1] = false;
    let mut ret = BTreeSet::new();
    for (i, &b) in v.iter().enumerate() {
        if b {
            ret.insert(i as u32);
        }
    }

    ret
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn setgcd(arr: &Vec<u32>) -> u32 {
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
    input!(aa: [u32; n]);

    let max = 1000000;
    if setgcd(&aa) == 1 {
        let ps = primes_upto(max);
        // println!("{:?}", ps);
        let mut used = BTreeSet::new();
        for a in aa {
            if ps.contains(&a) {
                used.insert(a);
                continue;
            }
            let mut a = a;
            for &p in ps.iter() {
                if a < p {
                    break;
                }
                if a % p == 0 {
                    if used.contains(&p) {
                        break;
                    }
                    used.insert(p);
                    while a % p == 0 {
                        a /= p;
                    }
                    if a == 1 {
                        break;
                    }
                }
            }
            // println!("ps = {:?}", ps);
            // println!("a = {}", a);
            if a != 1 {
                println!("setwise coprime");
                return;
            }
        }
        println!("pairwise coprime");
    } else {
        println!("not coprime");
    }
}
