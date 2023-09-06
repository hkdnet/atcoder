#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize);
    input!(aa: [i32; N]);
    input!(bb: [i32; N]);
    input!(cc: [i32; N]);
    let to_c = |v: &[i32]| {
        let mut ret = std::collections::BTreeMap::<i32, usize>::new();
        for i in v {
            let k = i % 46;
            ret.entry(k).and_modify(|a| *a += 1).or_insert(1);
        }

        ret
    };
    let ca = to_c(&aa);
    let cb = to_c(&bb);
    let cc = to_c(&cc);

    let mut ans = 0;
    for (&a, &c1) in ca.iter() {
        for (&b, &c2) in cb.iter() {
            let tmp = (a + b) % 46;
            let c = (46 - tmp) % 46;
            if let Some(c3) = cc.get(&c) {
                ans += c1 * c2 * c3;
            }
        }
    }

    println!("{}", ans);
}
