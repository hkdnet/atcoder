#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(H: usize, W: usize, _: usize);
    input!(T: Chars);
    let b = {
        let mut ret = vec![vec![]; H];
        for i in 0..H {
            input!(r: Chars);
            ret[i] = r;
        }
        ret
    };
    let mut ans = 0;
    for x in 0..H {
        for y in 0..W {
            if b[x][y] != '.' {
                continue;
            }
            let mut x = x;
            let mut y = y;

            let mut ok = true;
            for &d in T.iter() {
                let (dx, dy) = match d {
                    'L' => (0, !0),
                    'R' => (0, 1),
                    'D' => (1, 0),
                    'U' => (!0, 0),
                    _ => unreachable!(),
                };
                x = x.wrapping_add(dx);
                y = y.wrapping_add(dy);
                if x >= H || y >= W {
                    ok = false;
                    break;
                }
                if b[x][y] != '.' {
                    ok = false;
                    break;
                }
            }

            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
