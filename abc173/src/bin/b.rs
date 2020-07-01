fn main() {
    use proconio::input;
    use std::collections::BTreeMap;

    input!(n: usize);
    input!(ss: [String; n]);
    let mut cnt: BTreeMap<String, u32> = BTreeMap::new();
    for s in ss {
        *cnt.entry(s).or_default() += 1;
    }
    let e = vec!["AC", "WA", "TLE", "RE"];
    for e in e {
        let c = cnt.entry(e.to_string()).or_default();

        println!("{} x {}", e, c);
    }
}
