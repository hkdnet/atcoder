use std::collections::BTreeMap;

use comp::union_find::UnionFind;
use std::collections::BTreeSet;

fn main() {
    use proconio::input;
    use proconio::marker::Usize1;
    input!(n: usize, m: usize, k: usize);
    let builder = |n| {
        let mut ret: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for _ in 0..n {
            input!(a: Usize1, b: Usize1);
            ret.entry(a).or_insert(BTreeSet::new()).insert(b);
            ret.entry(b).or_insert(BTreeSet::new()).insert(a);
        }
        ret
    };
    let friends = builder(m);
    let blockers = builder(k);
    let mut uf = UnionFind::new(n);
    for (&a, bs) in friends.iter() {
        for &b in bs.iter() {
            uf.unite(a, b);
        }
    }

    let cnt = uf.counts();

    // println!("f {:?}", friends);
    // println!("b {:?}", blockers);
    // println!("{:?}", uf.v);
    let ans = (0..n)
        .map(|i| {
            let r = uf.root(i);
            let mut ans = *cnt.get(&r).unwrap() - 1;
            let direct_friends = friends.get(&i).map_or(0, |s| {
                // println!("direct friends: {:?}", s);
                s.len()
            });
            // println!("uf.v[33]    {}", uf.root(33));
            // println!("uf.v[29430] {}", uf.root(2930));
            // println!("33 cnt {:?}", cnt.get(&33));
            // println!("29430 cnt {:?}", cnt.get(&29430));
            // println!("29430 f {:?}", friends.get(&29430));
            // println!(
            //     "for {}: r = {}, ans = {}, direct_friends = {}",
            //     i, r, ans, direct_friends
            // );
            ans -= direct_friends;

            // println!("{:?}", blockers.get(&i));

            if let Some(bs) = blockers.get(&i) {
                for &b in bs {
                    if uf.root(b) == r {
                        ans -= 1;
                    }
                }
            }

            ans.to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
