use proconio::input;
use std::collections::BinaryHeap;

const K: usize = 2000000;

fn main() {
    input!(n: usize, q: usize);
    input!(abs: [(u32, usize); n]);
    input!(cds: [(usize, usize); q]);
    let mut heaps = vec![BinaryHeap::new(); K];

    let mut rates = vec![0; n];
    let mut children = vec![0; n];
    for i in 0..n {
        let (r, k) = abs[i];
        rates[i] = r;
        heaps[k].push(r);
        children[i] = k;
    }

    for (c, d) in cds {
        let now = children[c];
        children[c] = d;
        heaps[now].retain(|e| e != rates[c]);
    }

    println!("{:?}", abs);
    println!("{:?}", cds);
}
