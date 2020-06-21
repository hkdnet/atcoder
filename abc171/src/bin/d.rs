use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input!(n: usize);
    input!(aa: [i32; n]);
    input!(q: usize);
    input!(bcs: [(i32, i32); q]);

    let mut max = 0i128;
    let mut cnt: BTreeMap<i32, i128> = BTreeMap::new();

    for a in aa {
        max += a as i128;
        cnt.entry(a).and_modify(|e| *e += 1).or_insert(1);
    }
    for (b, c) in bcs {
        let delta = c - b;
        match cnt.remove(&b) {
            Some(k) => {
                max += k * (delta as i128);
                cnt.entry(c).and_modify(|e| *e += k).or_insert(k);
            }
            None => {}
        }

        println!("{}", max);
    }
}
