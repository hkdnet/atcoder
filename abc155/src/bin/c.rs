use proconio::input;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    input!(n: usize);
    input!(ss: [String; n]);
    let mut cnt: BTreeMap<String, u32> = BTreeMap::new();
    for s in ss {
        cnt.entry(s).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut max = 0;
    for &v in cnt.values() {
        if v > max {
            max = v;
        }
    }
    let mut s = BTreeSet::new();
    for (k, &v) in cnt.iter() {
        if v == max {
            s.insert(k);
        }
    }
    for k in s {
        println!("{}", k);
    }
}
