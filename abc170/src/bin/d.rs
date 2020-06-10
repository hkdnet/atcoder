use proconio::input;
use std::collections::{HashMap, HashSet};
fn main() {
    input!(n: usize); // 1<= n <= 2* 10**5
    input!(mut aa:  [u32; n]);

    aa.sort();
    let mut cnt = HashMap::new();

    for a in aa.iter() {
        cnt.entry(a).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut ans = 0;
    let mut divider = HashSet::new();
    for i in 0..n {
        if *cnt.get(&aa[i]).unwrap() > 1 {
            divider.insert(aa[i]);
            continue;
        }

        let mut ok = true;
        for e in divider.iter() {
            if aa[i] % e == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
            divider.insert(aa[i]);
        }
    }

    println!("{}", ans);
}
