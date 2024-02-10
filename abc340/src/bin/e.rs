#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]

use ac_library::Additive;
use ac_library::LazySegtree;
use ac_library::MapMonoid;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

struct RangeAdd;
impl MapMonoid for RangeAdd {
    type M = Additive<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &usize, &x: &usize) -> usize {
        f + x
    }

    fn composition(&f: &usize, &g: &usize) -> usize {
        f + g
    }
}

fn main() {
    input!(N: usize, M: usize);
    input!(a: [usize; N]);
    input!(B: [usize; M]);

    let mut s = LazySegtree::<RangeAdd>::new(N);
    for (i, v) in a.into_iter().enumerate() {
        s.set(i, v);
    }
    for i in B {
        let cnt = s.get(i);
        s.set(i, 0);
        let t = cnt / N;
        if t > 0 {
            s.apply_range(0..N, t);
        }
        let rest = cnt % N;
        if rest == 0 {
            continue;
        }
        let l = i + 1;
        let r = (l + rest) % N;
        if r < l {
            s.apply_range(l..N, 1);
            s.apply_range(0..r, 1);
        } else {
            s.apply_range(l..r, 1);
        }
    }
    let ans: Vec<_> = (0..N).map(|i| s.get(i).to_string()).collect();
    println!("{}", ans.join(" "));
}
