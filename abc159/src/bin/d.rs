use proconio::input;
use std::collections::HashMap;
fn main() {
    input!(n: usize);
    input!(aa: [usize; n]);
    let mut cnt = HashMap::new();

    for &a in aa.iter() {
        cnt.entry(a).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut sum = 0i128;

    for v in cnt.values() {
        sum += v * (v - 1) / 2;
    }

    for &a in aa.iter() {
        let c = cnt.get(&a).unwrap();
        let orig = c * (c - 1) / 2;
        let now = (c - 1) * (c - 2) / 2;
        let diff = orig - now;
        println!("{}", sum - diff);
    }
}
