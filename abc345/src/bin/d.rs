#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
use std::cmp::Reverse;

use proconio::input;
use proconio::marker::*;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
fn place_p(v: &Vec<Vec<bool>>, i: usize, j: usize, a: usize, b: usize) -> bool {
    if i + a - 1 < v.len() && j + b - 1 < v[0].len() {
        for di in 0..a {
            for dj in 0..b {
                if v[i + di][j + dj] {
                    return false;
                }
            }
        }
        true
    } else {
        false
    }
}
fn place(v: &mut [Vec<bool>], i: usize, j: usize, a: usize, b: usize, val: bool) {
    for di in 0..a {
        for dj in 0..b {
            v[i + di][j + dj] = val
        }
    }
}

fn show(v: &[Vec<bool>]) {
    for row in v.iter() {
        for &c in row.iter() {
            let ch = if c { '#' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }
    println!("---");
}
fn main() {
    input!(N: usize, H: usize, W: usize);
    input!(tiles: [(usize, usize); N]);
    let mut tiles = tiles;
    let s: usize = tiles.iter().map(|(a, b)| a * b).sum();
    if s < H * W {
        println!("No");
        return;
    }
    tiles.sort_unstable_by_key(|(a, b)| Reverse(a * b));
    let mut v = vec![vec![false; W]; H];

    fn f(v: &mut Vec<Vec<bool>>, tiles: &[(usize, usize)]) -> bool {
        if tiles.is_empty() {
            // show(&v);
            return v.iter().all(|vv| vv.iter().all(|&b| b));
        }
        let (a, b) = tiles[0];

        let pattern = if a == b {
            vec![(a, b)]
        } else {
            vec![(a, b), (b, a)]
        };
        for (a, b) in pattern.into_iter() {
            for i in 0..v.len() {
                for j in 0..v[0].len() {
                    if place_p(v, i, j, a, b) && (i == 0 || !v[i - 1][j]) {
                        place(v, i, j, a, b, true);
                        if f(v, &tiles[1..]) {
                            return true;
                        }
                        place(v, i, j, a, b, false);
                        continue;
                    }
                }
            }
        }

        // 使用しない
        if f(v, &tiles[1..]) {
            return true;
        }
        false
    }
    if f(&mut v, &tiles) {
        println!("Yes");
    } else {
        println!("No");
    }
}
