#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);
    input!(aa: [Usize1; M]);

    let mut counter = BTreeMap::<usize, usize>::new();
    let mut max = 0;
    let mut m = 0;

    for a in aa {
        counter.entry(a).and_modify(|e| *e += 1).or_insert(1);
        let &changed = counter.get(&a).unwrap();
        if max == changed {
            if a < m {
                m = a;
            }
        } else if max < changed {
            m = a;
            max = changed;
        }
        println!("{}", m + 1);
    }
}
