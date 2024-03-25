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
    input!(N: usize, M: usize);
    input!(x: [Usize1; M]);
    let dist = |a: usize, b: usize| -> (usize, usize) { (b - a, a + N - b) };
    let mut seg = LazySegtree::<RangeAdd>::new(N);
    for i in 0..M - 1 {
        let (a, b) = if x[i] < x[i + 1] {
            (x[i], x[i + 1])
        } else {
            (x[i + 1], x[i])
        };
        let (c, ant) = dist(a, b);
        if c < ant {
            // 時計周りのがはやい
            seg.apply_range(a..b, ant - c);
        }
        if ant < c {
            seg.apply_range(0..a, c - ant);
            seg.apply_range(b..N, c - ant);
        }
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
        f + x
    }

    fn composition(&f: &usize, &g: &usize) -> usize {
        f + g
    }
}
