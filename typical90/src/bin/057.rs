#![allow(unused_imports)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::*;

#[allow(non_snake_case)]
fn main() {
    input!(N: usize, M: usize);
    let mut switches = Vec::with_capacity(N);
    for _ in 0..N {
        input!(T: usize);
        input!(a: [Usize1; T]);
        let mut v = vec![false; M];
        for idx in a {
            v[idx] = true;
        }
        switches.push(v);
    }
    switches.sort_unstable();
    switches.reverse();

    for s in switches.iter() {
        println!("{:?}", s);
    }

    {
        let mut panel_idx = 0;
        let mut switch_idx = 0;
        while switch_idx < switches.len() {
            if !switches[switch_idx][panel_idx] {
                panel_idx += 1;
                continue;
            }
            let mut si = switch_idx + 1;
            while si < switches.len() && switches[si][panel_idx] {
                for j in panel_idx..M {
                    if switches[switch_idx][j] {
                        switches[si][j] = !switches[si][j];
                    }
                }
                si += 1;
            }
            panel_idx += 1;
            switch_idx = si + 1;
        }
    }

    for s in switches.iter() {
        println!("{:?}", s);
    }

    input!(expected: [char; M]);
    let mut expected = expected.iter().map(|&c| c == '1').collect::<Vec<_>>();
    let all_off_p = |expected: &Vec<bool>, l: usize, r: usize| -> bool {
        for i in l..r {
            if expected[i] {
                return false;
            }
        }
        true
    };
    let ans = {
        let mut idx = 0;
        let MOD = 998244353;
        let mut ans = 0i64;
        for i in 0..N {
            dbg!(&expected);
            if idx >= switches.len() {
                println!("swtiches ran out before reaching the end of expected");
                let all_off = all_off_p(&expected, i, N);

                if all_off {
                    println!("all off");
                    ans = 1;
                    for _ in i + 1..M {
                        ans *= 2;
                        ans %= MOD;
                    }
                } else {
                    println!("not all off");
                    ans = 0;
                }
                break;
            }

            if expected[i] {
                println!("a switch is required to turn off {}", i);
                println!("currently {} is checked", idx);
                while idx < switches.len() && !switches[idx][i] {
                    println!("{} is not usable {:?}", idx, &switches[idx]);
                    idx += 1;
                }
                if idx >= switches.len() {
                    println!("No switch is found. returning 0");
                    ans = 0;
                    break;
                }
                println!("Found {}", idx);
                dbg!(&switches[idx]);
                for j in i..N {
                    if switches[idx][j] {
                        expected[j] = !expected[j];
                    }
                }
                println!("-- ↓ --");
                dbg!(&expected);
                idx += 1;
            }
        }

        if idx < switches.len() {
            ans = 1;
            for _ in 1..M - idx {
                ans *= 2;
                ans %= MOD;
            }
        }
        ans
    };
    println!("{}", ans);
}
