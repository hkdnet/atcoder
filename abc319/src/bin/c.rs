#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(c: [i32; 9]);
    let mut c = c;
    c.insert(0, 0);
    let all = 362880.0f64;

    let mut cnt = 0;

    let vv: Vec<Vec<usize>> = [
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [1, 5, 9],
        [3, 5, 7],
    ]
    .into_iter()
    .filter_map(|v| {
        if c[v[0]] == c[v[1]] {
            Some(vec![v[0], v[1], v[2]])
        } else if c[v[1]] == c[v[2]] {
            Some(vec![v[1], v[2], v[0]])
        } else if c[v[2]] == c[v[0]] {
            Some(vec![v[2], v[0], v[1]])
        } else {
            None
        }
    })
    .collect();

    for v in (1..=9).permutations(9) {
        let mut f = false;
        for a in vv.iter() {
            let tmp: Vec<&usize> = v
                .iter()
                .filter(|&i| *i == a[0] || *i == a[1] || *i == a[2])
                .collect();
            if *tmp[2] == a[2] {
                f = true;
            }
            if f {
                break;
            }
        }
        if !f {
            cnt += 1;
        }
    }

    println!("{}", cnt as f64 / all);
}
