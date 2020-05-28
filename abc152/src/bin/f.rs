use proconio::input;
use std::collections::{HashMap, HashSet};

fn range(tree: &HashMap<usize, usize>, a: usize, b: usize) -> HashSet<usize> {
    let mut ret = HashSet::new();
    let mut tmp = a;
    ret.insert(tmp);
    loop {
        match tree.get(&tmp) {
            Some(&n) => {
                tmp = n;
                ret.insert(tmp);
            }
            None => {
                // root
                break;
            }
        }
    }
    tmp = b;
    loop {
        if ret.contains(&tmp) {
            break;
        }
        match tree.get(&tmp) {
            Some(&n) => {
                tmp = n;
                ret.insert(tmp);
            }
            None => {
                // root
                break;
            }
        }
    }

    ret
}
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
    input!(n: usize);
    input!(edges: [(usize, usize); n - 1]);
    input!(m: usize);
    input!(consts: [(usize, usize); m]);
    // 余事象 2 ** (n-1) - (条件を満たさないものの数)
    // NOT(M個の制約をすべて満たす) -> 与えられている制約区間が全部白い
    // 区間のマージをすればよいが区間はたかだか 20 個までなのでなんとかなりそう
    let mut parents = HashMap::new();
    for i in 0..n - 1 {
        let (a, b) = edges[i];
        if a < b {
            parents.insert(a - 1, b - 1);
        } else {
            parents.insert(b - 1, a - 1);
        }
    }

    let mut uf = UnionFind::new(n);

    for i in 0..m {
        let (a, b) = consts[i];
        let rr: Vec<usize> = range(&parents, a - 1, b - 1).into_iter().collect();
        // rr.size() > 1 は所与
        let g = rr[0];
        for r in rr {
            uf.union(g, r);
        }
    }

    uf.flatten();

    println!("{:?}", uf.v);
    let mut uniq = HashSet::<usize>::new();
    for r in uf.v.iter() {
        uniq.insert(*r);
    }

    let all = 2i32.pow((n - 1) as u32);
    let all_whites = 2i32.pow((uniq.len() - 1) as u32);
    //     println!("all = {}, all_whites = {}", all, all_whites);
    println!("{}", all - all_whites);
}
