fn main() {
    use proconio::input;
    input!(k: u64);

    use std::collections::BTreeSet;
    let mut delta = 7 % k;
    let mut m = 7 % k;
    let mut used: BTreeSet<u64> = BTreeSet::new();

    while m != 0 {
        delta *= 10;
        delta %= k;
        m += delta;
        m %= k;
        if used.contains(&m) {
            println!("-1");
            return;
        }
        used.insert(m);
    }
    println!("{}", used.len() + 1);
}
