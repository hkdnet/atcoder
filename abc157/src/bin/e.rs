use std::ops::Bound::Included;

fn main() {
    use proconio::input;
    use proconio::marker::Chars;
    use proconio::marker::Usize1;
    input!(_: usize, mut s: Chars, q: usize);
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    let mut cs: BTreeMap<char, BTreeSet<usize>> = BTreeMap::new();
    let all_chars = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    for &c in all_chars.iter() {
        cs.insert(c, BTreeSet::new());
    }
    for (idx, &c) in s.iter().enumerate() {
        cs.entry(c).and_modify(|se| {
            se.insert(idx);
        });
    }

    for _ in 0..q {
        input!(ty: usize);
        if ty == 1 {
            input!(i: Usize1, c: char);
            let old = s[i];
            s[i] = c;
            cs.entry(old).and_modify(|se| {
                se.remove(&i);
            });
            cs.entry(c).and_modify(|se| {
                se.insert(i);
            });
        } else {
            input!(l: Usize1, r: Usize1);
            let mut cnt = 0;
            for &c in all_chars.iter() {
                cs.entry(c).and_modify(|se| {
                    for _ in se.range((Included(&l), Included(&r))) {
                        cnt += 1;
                        break;
                    }
                });
            }
            println!("{}", cnt);
        }
    }
}
