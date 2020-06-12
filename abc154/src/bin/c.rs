use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input!(n: usize);
    input!(aa: [u32; n]);

    let set: BTreeSet<u32> = aa.into_iter().collect();
    if set.len() == n {
        println!("YES");
    } else {
        println!("NO");
    }
}
