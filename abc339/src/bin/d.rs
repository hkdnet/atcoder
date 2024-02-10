#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::collections::BTreeMap;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input!(N: usize);
    let b = {
        let mut v = vec![vec![]; N];
        for i in 0..N {
            input!(cs: Chars);
            v[i] = cs;
        }
        v
    };
    let s = {
        let mut p1 = None;
        let mut p2 = None;
        for x in 0..N {
            for y in 0..N {
                if b[x][y] == 'P' {
                    if p1.is_none() {
                        p1 = Some((x, y));
                    } else {
                        p2 = Some((x, y));
                    }
                }
            }
        }
        let (x1, y1) = p1.unwrap();
        let (x2, y2) = p2.unwrap();
        (x1, y1, x2, y2)
    };
    let mut q = VecDeque::from([s]);
    let mut c = BTreeMap::from([(s, 0)]);
    while let Some(cur) = q.pop_front() {
        let (x1, y1, x2, y2) = cur;
        let cnt = *c.get(&cur).unwrap();
        for &(dx, dy) in &[(1usize, 0usize), (!0, 0), (0, 1), (0, !0)] {
            let mut nx1 = x1.wrapping_add(dx);
            let mut ny1 = y1.wrapping_add(dy);
            if nx1 >= N || ny1 >= N || b[nx1][ny1] == '#' {
                nx1 = x1;
                ny1 = y1;
            }

            let mut nx2 = x2.wrapping_add(dx);
            let mut ny2 = y2.wrapping_add(dy);
            if nx2 >= N || ny2 >= N || b[nx2][ny2] == '#' {
                nx2 = x2;
                ny2 = y2;
            }
            let nx = (nx1, ny1, nx2, ny2);
            if nx1 == nx2 && ny1 == ny2 {
                println!("{}", cnt + 1);
                return;
            }

            c.entry(nx).or_insert_with(|| {
                q.push_back(nx);
                cnt + 1
            });
        }
    }
    println!("{}", -1);
}
