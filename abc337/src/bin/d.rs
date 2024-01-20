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
fn required_op(k: usize, cs: &[char]) -> Option<usize> {
    let mut l = 0;
    let mut ret = None;
    while (l + k - 1) < cs.len() {
        let mut tmp = Some(0);
        for d in 0..k {
            match cs[l + d] {
                'x' => {
                    tmp = None;
                    break;
                }
                '.' => {
                    tmp = tmp.map(|e| e + 1);
                }
                _ => {}
            }
            if tmp.is_none() {
                break;
            }
        }
        if let Some(v) = tmp {
            ret = Some(ret.map_or(v, |e| std::cmp::min(e, v)));
        }
        l += 1;
    }
    ret
}

fn main() {
    input!(H: usize, W: usize, K: usize);
    let v = {
        let mut v = Vec::with_capacity(H);
        for _ in 0..H {
            input!(cs: Chars);
            v.push(cs);
        }
        v
    };
    let mut ans = None;
    for row in v.iter() {
        if let Some(v) = required_op(K, row) {
            ans = Some(ans.map_or(v, |e| std::cmp::min(e, v)));
        }
    }
    for w in 0..W {
        let row: Vec<char> = (0..H).map(|x| v[x][w]).collect();
        if let Some(v) = required_op(K, &row) {
            ans = Some(ans.map_or(v, |e| std::cmp::min(e, v)));
        }
    }
    if let Some(v) = ans {
        println!("{}", v);
    } else {
        println!("-1");
    }
}
