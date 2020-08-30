#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
fn main() {
    input!(cc: Chars, q: usize);
    let mut d = VecDeque::new();
    for c in cc {
        d.push_back(c);
    }

    let mut reversed = false;

    for _ in 0..q {
        input!(ty: usize);
        if ty == 1 {
            reversed = !reversed;
        } else {
            input!(f: usize, c: char);
            let is_front = f == 1;
            match (is_front, reversed) {
                (true, false) | (false, true) => {
                    // 前から
                    d.push_front(c);
                }
                _ => {
                    d.push_back(c);
                }
            }
        }
    }
    let output = if reversed {
        d.iter().rev().collect::<String>()
    } else {
        d.iter().collect::<String>()
    };
    println!("{}", output);
}
