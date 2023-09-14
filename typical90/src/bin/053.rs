#![allow(unused_imports)]
use std::collections::BTreeMap;
use std::process::exit;

use proconio::input;
use proconio::marker::*;

pub fn binary_search<T, F>(l: T, r: T, mut f: F) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
    F: FnMut(T) -> bool,
{
    let mut l = l;
    let mut r = r;

    loop {
        let idx = (l + r) / T::from(2u8);
        // idx == r is required when l > r.
        if idx == l || idx == r {
            break;
        }
        if f(idx) {
            l = idx;
        } else {
            r = idx;
        }
    }

    l
}

#[allow(non_snake_case)]
fn main() {
    input!(T: usize);
    for _ in 0..T {
        match solve() {
            Ok(v) => {
                println!("! {}", v);
            }
            Err(_) => {
                exit(0);
            }
        }
    }
}

fn solve() -> Result<i32, ()> {
    input!(N: usize);
    let mut memo = BTreeMap::new();
    let mut get_val = |i: usize| -> Result<i32, ()> {
        if memo.contains_key(&i) {
            return Ok(*memo.get(&i).unwrap());
        }
        println!("? {}", i);
        input!(val: i32);
        if val == -1 {
            Err(())
        } else {
            memo.insert(i, val);
            Ok(val)
        }
    };
    let cmp = |i: usize| -> bool {
        let v1 = get_val(i).unwrap();
        let v2 = get_val(i + 1).unwrap();
        v1 < v2
    };
    let idx = binary_search(0, N, cmp);
    get_val(idx + 1)
}
