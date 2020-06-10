use proconio::input;
use std::collections::HashMap;
fn main() {
    input!(n: usize); // 1<= n <= 2* 10**5
    input!(mut aa:  [u32; n]);

    aa.sort();
    let mut cnt = HashMap::new();

    for a in aa.iter() {
        cnt.entry(a).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut ans = 0;
    for i in 0..n {
        if *cnt.get(&aa[i]).unwrap() > 1 {
            continue;
        }

        let mut ok = true;
        for j in 0..i {
            if aa[i] % aa[j] == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
