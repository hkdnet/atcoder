#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Add;

use ac_library::Monoid;
use proconio::input;
use proconio::marker::*;

use ac_library::Additive;
use ac_library::LazySegtree;
use ac_library::MapMonoid;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize, Q: usize);
    input!(s: Chars);
    let mut v = vec![1; N];
    for i in 1..N {
        v[i] = if s[i] == s[i - 1] { 0 } else { 1 };
    }

    let mut seg = LazySegtree::<RangeAdd>::new(N);
    for (i, n) in v.into_iter().enumerate() {
        seg.set(i, n);
    }

    for _ in 0..Q {
        input!(ty: usize, l: usize, r: usize);
        if ty == 1 {
            for &i in &[l + 1, r + 1] {
                if i >= N {
                    continue;
                }
                let v = seg.get(i);
                seg.set(i, (v + 1) % 2);
            }
            for i in 0..N {
                print!("{}", seg.get(i));
            }
            println!("");
        } else {
            let expected = r - l;
            let ok = seg.prod(l..r) == expected;
            debug!(l, r, expected, ok);
            if ok {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

struct RangeAdd;
impl MapMonoid for RangeAdd {
    type M = Additive<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &usize, &x: &usize) -> usize {
        (f + x) % 2
    }

    fn composition(&f: &usize, &g: &usize) -> usize {
        (f + g) % 2
    }
}
