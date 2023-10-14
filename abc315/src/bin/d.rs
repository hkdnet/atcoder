#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(H: usize, W: usize);
    let mut c = Vec::with_capacity(H);
    let mut h = BTreeMap::<usize, BTreeMap<char, usize>>::new();
    let mut w = BTreeMap::<usize, BTreeMap<char, usize>>::new();
    for _ in 0..H {
        input!(r: Chars);
        c.push(r);
    }
    for i in 0..H {
        let mut m = BTreeMap::<char, usize>::new();
        for j in 0..W {
            let cc = c[i][j];
            m.entry(cc)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }
        h.insert(i, m);
    }
    for i in 0..W {
        let mut m = BTreeMap::<char, usize>::new();
        for j in 0..H {
            let cc = c[j][i];
            m.entry(cc)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }
        w.insert(i, m);
    }

    loop {
        let mut changed_h = vec![];
        let mut changed_w = vec![];
        for (&idx, hh) in h.iter() {
            if hh.len() == 1 {
                let (&c, &v) = hh.first_key_value().unwrap();
                if v > 1 {
                    changed_h.push((c, idx));
                }
            }
        }
        for (&idx, ww) in w.iter() {
            if ww.len() == 1 {
                let (&c, &v) = ww.first_key_value().unwrap();
                if v > 1 {
                    changed_w.push((c, idx));
                }
            }
        }

        if changed_h.is_empty() && changed_w.is_empty() {
            break;
        }

        for (c, hi) in changed_h {
            h.remove(&hi);
            for ww in w.values_mut() {
                let mut f = false;
                ww.entry(c).and_modify(|e| {
                    *e -= 1;
                    f = *e == 0;
                });
                if f {
                    ww.remove(&c);
                }
            }
        }
        for (c, wi) in changed_w {
            w.remove(&wi);
            for hh in h.values_mut() {
                let mut f = false;
                hh.entry(c).and_modify(|e| {
                    *e -= 1;
                    f = *e == 0;
                });
                if f {
                    hh.remove(&c);
                }
            }
        }
    }
    let hsum: usize = h.values().map(|hh| hh.values().sum::<usize>()).sum();

    println!("{}", hsum);
}
