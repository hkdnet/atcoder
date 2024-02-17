#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use ac_library::Additive;
use ac_library::LazySegtree;
use ac_library::MapMonoid;
use ac_library::Max;
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<u64>;
    type F = u64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &u64, &x: &u64) -> u64 {
        f + x
    }

    fn composition(&f: &u64, &g: &u64) -> u64 {
        f + g
    }
}
fn main() {
    input!(N: usize, D: usize);
    input!(A: [usize; N]);

    let mut seg = LazySegtree::<MaxAdd>::new(1_000_002);
    for &a in A.iter() {
        let l = if a < D { 0 } else { a - D };
        let r = a + D;
        let m = seg.prod(l..=r);
        seg.set(a, m + 1);
    }

    let ans = seg.all_prod();
    println!("{}", ans);
}
