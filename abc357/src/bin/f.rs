#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::convert::Infallible;
use std::marker::PhantomData;

use ac_library::Monoid;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
const MOD: u64 = 998244353;

fn main() {
    input!(N: usize, Q: usize, A: [u64; N], B: [u64; N]);

    let mut seg = LazySegtree::<PairAdd>::new(N);
    for (i, (a, b)) in A.into_iter().zip(B).enumerate() {
        seg.set(i, (a, b, 1));
    }
    for _ in 0..Q {
        input!(ty: usize);
        if ty == 1 {
            input!(l: Usize1, r: Usize1, x: u64);
            seg.apply_range(l..=r, (x, 0));
        } else if ty == 2 {
            input!(l: Usize1, r: Usize1, x: u64);
            seg.apply_range(l..=r, (0, x));
        } else {
            input!(l: Usize1, r: Usize1);
            let (a, b, _) = seg.prod(l..=r);
            debug!(seg);
            println!("{}", (a * b) % MOD)
        }
    }
}

use ac_library::Additive;
use ac_library::LazySegtree;
use ac_library::MapMonoid;
use ac_library::Max;

type Val = (u64, u64, u64, u64);
pub struct Top2<S>(Infallible, PhantomData<fn() -> S>);
impl Monoid for Top2<Val> {
    type S = Val;
    fn identity() -> Self::S {
        (0, 0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let (a1, a2, a_size) = a;
        let (b1, b2, b_size) = b;
        let v = ((*a1 * *a2) % MOD + (*b1 * *b2) % MOD) % MOD;
        (v, 1, a_size + b_size)
    }
}

struct PairAdd;
impl MapMonoid for PairAdd {
    type M = Top2<Val>;
    type F = (u64, u64);

    fn identity_map() -> Self::F {
        (0, 0)
    }

    fn mapping(&f: &Self::F, x: &Val) -> Val {
        let (f1, f2) = f;
        let (x1, x2, x_size) = x;
        ((x1 + f1 * x_size) % MOD, (x2 + f2 * x_size) % MOD, *x_size)
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        let (f1, f2) = f;
        let (g1, g2) = g;
        ((f1 + g1) % MOD, (f2 + g2) % MOD)
    }
}
