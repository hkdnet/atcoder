#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::marker::PhantomData;

use ac_library::Monoid;
use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use ac_library::Additive;
use ac_library::LazySegtree;
use ac_library::MapMonoid;
use ac_library::Max;

pub trait BoundedBelow {
    fn min_value() -> Self;
}
type Val = Vec<(u64, usize)>;
pub struct Top2<S>(Infallible, PhantomData<fn() -> S>);
impl Monoid for Top2<Val> {
    type S = Val;
    fn identity() -> Self::S {
        vec![]
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let mut m = BTreeMap::new();
        for &(i, cnt) in a.iter() {
            m.entry(i)
                .and_modify(|e| {
                    *e += cnt;
                })
                .or_insert(cnt);
        }
        for &(i, cnt) in b.iter() {
            m.entry(i)
                .and_modify(|e| {
                    *e += cnt;
                })
                .or_insert(cnt);
        }

        // debug!(a, b, m);

        let f = |k: &u64| {
            let &v = m.get(k).unwrap();
            (*k, v)
        };
        m.keys().rev().take(2).map(f).collect_vec()
    }
}

struct Replace;
impl MapMonoid for Replace {
    type M = Top2<Val>;
    type F = Option<u64>;

    fn identity_map() -> Self::F {
        None
    }

    fn mapping(&f: &Option<u64>, x: &Val) -> Val {
        if let Some(v) = f {
            vec![(v, 1)]
        } else {
            x.to_vec()
        }
    }

    fn composition(&f: &Option<u64>, &g: &Option<u64>) -> Option<u64> {
        g
    }
}
fn main() {
    input!(N: usize, Q: usize);
    let mut seg = LazySegtree::<Replace>::new(N);
    input!(A: [u64; N]);
    for (i, a) in A.into_iter().enumerate() {
        seg.set(i, vec![(a, 1)]);
    }
    for _ in 0..Q {
        input!(ty: usize, a: usize, b: usize);
        if ty == 1 {
            let p = a - 1;
            let x = b as u64;
            seg.set(p, vec![(x, 1)]);
        } else {
            let l = a - 1;
            let r = b - 1;
            let v = seg.prod(l..=r);
            // debug!(v);
            if v.len() == 1 {
                println!("0");
            } else {
                println!("{}", v[1].1);
            }
        }
    }
    // println!("---");
    // let mut segv = vec![];
    // for i in 0..N {
    //     segv.push(seg.get(i));
    // }
    // debug!(segv);
    // println!("---");
}
