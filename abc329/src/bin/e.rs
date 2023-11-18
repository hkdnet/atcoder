#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(N: usize, M: usize);
    input!(S: Chars);
    input!(T: Chars);
    let start_index = {
        let mut cur = 0;
        let mut f = true;
        while f {
            let mut ok = true;
            for (i, &c) in T.iter().enumerate() {
                let idx = cur + i;
                if idx >= S.len() {
                    cur = N;
                    ok = false;
                    f = false;
                    break;
                }
                if S[idx] != c {
                    ok = false;
                    break;
                }
            }
            if ok {
                break;
            } else {
                cur += 1;
            }
        }
        cur
    };
    dbg!(start_index);
    if start_index >= N {
        println!("No");
        return;
    }

    let last = start_index + M;
    if last < N {
        if !ok(&S[last..], &T) {
            println!("No");
            return;
        }
    }
    if start_index != 0 {
        let mut cs: Vec<char> = S[0..start_index].to_vec();
        cs.reverse();

        let mut reversed = T.to_vec();
        reversed.reverse();
        if !ok(&cs, &reversed) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn ok(s: &[char], t: &[char]) -> bool {
    let mut v = vec![false; s.len()];
    dbg!(&s, &t);
    v[0] = true;
    for l in 0..v.len() {
        if !v[l] {
            continue;
        }
        for i in 0..t.len() {
            let j = i + 1;
            if l + j >= s.len() {
                break;
            }
            let mut ok = true;
            for i in 0..j {
                if s[i] != t[t.len() - i - 1] {
                    ok = false;
                    break;
                }
            }
            if ok {
                v[i] = true;
            }
        }
    }
    v[s.len() - 1]
}
