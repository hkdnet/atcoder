#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize);
    input!(es: [Usize1; N]);
    let mut ds = vec![None; N];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0));
    while let Some((dst, d)) = q.pop_front() {
        if ds[dst].is_some() {
            break;
        }
        if dst == 1 {
            println!("{}", d);
            return;
        }
        ds[dst] = Some(d);
        q.push_back((es[dst], d + 1));
    }

    println!("-1");
}
