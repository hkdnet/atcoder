use proconio::input;
use std::collections::{HashMap, HashSet};

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input!(n: usize); // 1<= n <= 2* 10**5
    input!(mut aa: [u32; n]);

    aa.sort();
    let mut cnt = HashMap::new();

    let mut g = aa[0];
    for a in aa.iter() {
        cnt.entry(a).and_modify(|e| *e += 1).or_insert(1);
        g = gcd(g, *a);
    }

    let mut ans = 0;
    let mut divider = HashSet::new();
    for i in 0..n {
        let a = aa[i] / g;

        if *cnt.get(&aa[i]).unwrap() > 1 {
            divider.insert(a);
            continue;
        }

        let mut ok = true;
        for e in divider.iter() {
            if a % e == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
            divider.insert(a);
        }
    }

    println!("{}", ans);
}
