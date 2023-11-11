#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input!(cs: Chars);

    let mut stack = VecDeque::new();
    for c in cs {
        stack.push_back(c);
        if stack.len() >= 3 {
            match (
                stack.get(stack.len() - 3),
                stack.get(stack.len() - 2),
                stack.get(stack.len() - 1),
            ) {
                (Some(&'A'), Some(&'B'), Some(&'C')) => {
                    stack.pop_back();
                    stack.pop_back();
                    stack.pop_back();
                }
                _ => {}
            }
        }
    }

    println!("{}", stack.into_iter().collect::<String>());
}
