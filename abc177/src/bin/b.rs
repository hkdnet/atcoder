#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
fn main() {
    input!(s: Chars, t: Chars);
    let sl = s.len();
    let tl = t.len();
    let mut min = tl;
    for si in 0..sl {
        if si + tl > sl {
            break;
        }
        let mut tmp = tl;
        for (idx, &tc) in t.iter().enumerate() {
            let sidx = si + idx;
            if s[sidx] == tc {
                tmp -= 1;
            }
        }
        min = std::cmp::min(min, tmp);
    }

    println!("{}", min);
}
