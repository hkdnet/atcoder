#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;
use proconio::marker::*;
#[derive(Debug)]
struct P {
    cost: i64,
    parameters: Vec<usize>,
}
impl P {
    fn add(&self, cur: usize, threshold: usize) -> usize {
        let ds: Vec<usize> = to_v(cur, self.parameters.len());
        let v = add(&self.parameters, &ds, threshold);
        // dbg!(&ds, &self.parameters, &v);
        v_to_usize(&v)
    }
}

fn add(a: &[usize], b: &[usize], threshold: usize) -> Vec<usize> {
    a.iter()
        .zip(b.iter())
        .map(|(aa, bb)| std::cmp::min(aa + bb, threshold))
        .collect()
}

fn to_v(a: usize, size: usize) -> Vec<usize> {
    let mut a = a;
    let mut v = vec![0; size];
    for i in 0..size {
        let d = a % 10;
        a /= 10;
        v[size - i - 1] = d;
    }
    v
}
fn v_to_usize(a: &Vec<usize>) -> usize {
    let mut mul = 1;
    for _ in 0..a.len() {
        mul *= 10;
    }
    mul /= 10;
    let mut ret = 0;
    for &v in a.iter() {
        ret += v * mul;
        mul /= 10;
    }
    ret
}

fn main() {
    // for i in 0..60000 {
    //     let got = v_to_usize(&to_v(i));
    //     if got != i {
    //         println!("{} vs {}", i, got);
    //         unreachable!()
    //     }
    // }

    input!(N: usize, K: usize, P: usize);
    let plans = {
        let mut plans = Vec::with_capacity(N);
        for _ in 0..N {
            input!(c: i64);
            input!(aa: [usize; K]);
            plans.push(P {
                cost: c,
                parameters: aa,
            });
        }
        plans
    };
    let mut dp = BTreeMap::new();
    dp.insert(0, 0i64);
    for plan in plans {
        let mut tmp = BTreeMap::new();
        for (&k, &v) in dp.iter() {
            tmp.entry(k)
                .and_modify(|cur_cost| {
                    *cur_cost = std::cmp::min(*cur_cost, v);
                })
                .or_insert(v);
            let nx_key = plan.add(k, P);
            let nx_val = v + plan.cost;
            // if nx_val == 4 {
            //     dbg!(k, v, nx_key, &plan);
            // }
            tmp.entry(nx_key)
                .and_modify(|cur_cost| {
                    *cur_cost = std::cmp::min(*cur_cost, nx_val);
                })
                .or_insert(nx_val);
        }
        std::mem::swap(&mut dp, &mut tmp);
    }
    let expected = {
        let mut v = 0;
        for i in 0..K {
            let mut delta = P;
            for _ in 0..i {
                delta *= 10;
            }
            v += delta;
        }
        v
    };
    // dbg!(expected, &dp);
    if let Some(cost) = dp.get(&expected) {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}
