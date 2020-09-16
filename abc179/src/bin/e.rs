#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(n: usize, x: u64, m: u64);
    let mut s = std::collections::BTreeSet::new();
    let mut arr = vec![0; (m+1) as usize];
    s.insert(x);
    arr[0] = x;
    let mut idx = 0;
    loop {
        let a = arr[idx];
        let v = (a * a) % m;
        arr[idx + 1] = v;
        if s.contains(&v) {
            break ;
        }
        s.insert(v);
        idx += 1;
    }
    // println!("s = {:?}, idx = {}", s, idx);
    let loop_end = idx + 1;
    let mut sum = 0;
    let mut loop_start = 0;
    while arr[loop_start] != arr[loop_end] {
        sum += arr[loop_start];
        loop_start += 1;
    }
    // println!("{} - {}", loop_start, loop_end);
    let loop_len = loop_end - loop_start;
    let rest = n - loop_start;
    let loop_cnt = rest / loop_len;
    let rest = rest % loop_len;
    let mut loop_sum = 0;
    for i in loop_start..loop_end {
        loop_sum += arr[i];
        if i < loop_start + rest {
            sum += arr[i];
        }
    }
    sum += loop_sum * (loop_cnt as u64);

    // println!("arr = {:?}", arr);
    // println!("loop [{}, {}] == len {} -> loop_cnt = {}\nrest = {}", loop_start,loop_end, loop_len, loop_cnt, rest);

    println!("{}", sum);
}
