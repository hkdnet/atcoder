use std::collections::BTreeMap;

use std::collections::BTreeSet;

struct UnionFind {
    v: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            v: (0..n).collect(),
        }
    }
    fn root(self: &mut Self, a: usize) -> usize {
        let mut tmp = a;
        let mut visited = vec![];
        while self.v[tmp] != tmp {
            visited.push(tmp);
            tmp = self.v[tmp];
        }

        for i in visited {
            self.v[i] = tmp;
        }

        tmp
    }
    fn union(self: &mut Self, a: usize, b: usize) {
        let ra = self.root(a);
        let rb = self.root(b);
        if ra != rb {
            self.v[b] = a;
        }
    }
    fn flatten(self: &mut Self) {
        for i in 0..self.v.len() {
            self.root(i);
        }
    }
}

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
            uf.union(a, b);
        }
    }
    uf.flatten();

    let cnt = {
        let mut c = BTreeMap::new();
        for &v in uf.v.iter() {
            c.entry(v).and_modify(|e| *e += 1).or_insert(1);
        }
        c
    };
    // println!("f {:?}", friends);
    // println!("b {:?}", blockers);
    // println!("{:?}", uf.v);
    let ans = (0..n)
        .map(|i| {
            let r = uf.root(i);
            let mut ans = *cnt.get(&r).unwrap() - 1;
            let direct_friends = friends.get(&i).map_or(0, |s| s.len());
            // println!(
            //     "for {}: r = {}, ans = {}, direct_friends = {}",
            //     i, r, ans, direct_friends
            // );
            ans -= direct_friends;

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
