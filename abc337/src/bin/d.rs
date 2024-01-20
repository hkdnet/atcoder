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
fn required_op(k: usize, cs: &[char]) -> usize {
    let acc_d = {
        let mut acc = vec![0; cs.len() + 1];
        for i in 0..cs.len() {
            acc[i + 1] = acc[i];
            if cs[i] == '.' {
                acc[i + 1] += 1;
            }
        }
        acc
    };
    let acc_x = {
        let mut acc = vec![0; cs.len() + 1];
        for i in 0..cs.len() {
            acc[i + 1] = acc[i];
            if cs[i] == 'x' {
                acc[i + 1] += 1;
            }
        }
        acc
    };
    let mut ret = 100000000;

    for l in 0..cs.len() {
        if l + k >= acc_x.len() {
            break;
        }
        let x = acc_x[l + k] - acc_x[l];
        if x != 0 {
            continue;
        }
        let d = acc_d[l + k] - acc_d[l];
        ret = std::cmp::min(d, ret);
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
    let mut ans = 100000000;
    for row in v.iter() {
        let v = required_op(K, row);
        if v != 100000000 {
            ans = std::cmp::min(ans, v);
        }
    }
    for w in 0..W {
        let row: Vec<char> = (0..H).map(|x| v[x][w]).collect();
        let v = required_op(K, &row);
        if v != 100000000 {
            ans = std::cmp::min(ans, v);
        }
    }
    if ans == 100000000 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
