fn main() {
    use proconio::input;
    use std::collections::BTreeMap;

    input!(n: u32);

    let mut m: BTreeMap<u32, usize> = BTreeMap::new();
    for x in 1..100 {
        for y in 1..100 {
            for z in 1..100 {
                let tmp = x * x + y * y + z * z + x * y + y * z + z * x;
                if tmp <= n {
                    *m.entry(tmp).or_default() += 1;
                }
            }
        }
    }

    for i in 1..n + 1 {
        let cnt = m.get(&i).unwrap_or(&0);
        println!("{}", cnt);
    }
}
