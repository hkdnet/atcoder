#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

use std::collections::BTreeMap;

const MOD: usize = 1_000_000_007;

fn main() {
    input!(n: usize, b: i32, k: usize);
    input!(ds: [i32; k]);
    let mut ds = ds;
    let mut mods: BTreeMap<i32, usize> = BTreeMap::new();
    for &d in ds.iter() {
        let key = d % b;
        mods.entry(key).and_modify(|e| *e += 1).or_insert(1);
    }
    for _ in 1..n {
        let mut new_ds = vec![0; ds.len()];
        for (idx, &d) in ds.iter().enumerate() {
            new_ds[idx] = (d * 10) % b;
        }
        std::mem::swap(&mut ds, &mut new_ds);
        let mut tmp = BTreeMap::new();

        for (&key, &cnt) in mods.iter() {
            for &d in ds.iter() {
                let new_key = (key + d) % b;
                tmp.entry(new_key)
                    .and_modify(|e| {
                        *e += cnt;
                        *e %= MOD;
                    })
                    .or_insert(cnt);
            }
        }
        std::mem::swap(&mut mods, &mut tmp);
    }

    println!("{}", mods.get(&0).unwrap_or(&0));
}
