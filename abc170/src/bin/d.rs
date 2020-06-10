use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut mm = BTreeSet::new();
    let mut mult = BTreeSet::new();

    for a in a {
        if mm.contains(&a) {
            mult.insert(a);
        } else {
            mm.insert(a);
        }
    }

    let mut ans = 0;

    let v = mm.iter().cloned().collect_vec();
    for x in v {
        if !mm.contains(&x) {
            continue;
        }

        if !mult.contains(&x) {
            ans += 1;
        }
        let mut y = x;
        while y <= 1_000_000 {
            mm.remove(&y);
            y += x;
        }
    }

    println!("{}", ans);
}
