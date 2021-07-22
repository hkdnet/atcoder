#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;

const MOD: i64 = 1_000_000_007;

fn main() {
    input!(_: usize, cs: Chars);
    let mut m = BTreeMap::new();
    for c in cs {
        match c {
            'a' => {
                m.entry('a')
                    .and_modify(|c| {
                        *c += 1;
                        *c %= MOD;
                    })
                    .or_insert(1);
            }
            't' => {
                let &cnt = m.get(&'a').unwrap_or(&0);
                m.entry('t')
                    .and_modify(|c| {
                        *c += cnt;
                        *c %= MOD;
                    })
                    .or_insert(cnt);
            }
            'c' => {
                let &cnt = m.get(&'t').unwrap_or(&0);
                m.entry('c')
                    .and_modify(|c| {
                        *c += cnt;
                        *c %= MOD;
                    })
                    .or_insert(cnt);
            }
            'o' => {
                let &cnt = m.get(&'c').unwrap_or(&0);
                m.entry('o')
                    .and_modify(|c| {
                        *c += cnt;
                        *c %= MOD;
                    })
                    .or_insert(cnt);
            }
            'd' => {
                let &cnt = m.get(&'o').unwrap_or(&0);
                m.entry('d')
                    .and_modify(|c| {
                        *c += cnt;
                        *c %= MOD;
                    })
                    .or_insert(cnt);
            }
            'e' => {
                let &cnt = m.get(&'d').unwrap_or(&0);
                m.entry('e')
                    .and_modify(|c| {
                        *c += cnt;
                        *c %= MOD;
                    })
                    .or_insert(cnt);
            }
            'r' => {
                let &cnt = m.get(&'e').unwrap_or(&0);
                m.entry('r')
                    .and_modify(|c| {
                        *c += cnt;
                        *c %= MOD;
                    })
                    .or_insert(cnt);
            }
            _ => {}
        }
    }

    println!("{}", m.get(&'r').unwrap_or(&0))
}
