use proconio::input;

struct U {
    n: usize,
    rates: Vec<u32>,
    children: Vec<u32>,
}

impl U {
    fn new(n: usize, rates: Vec<u32>, children: Vec<u32>) -> U {
        U {
            n: n,
            rates: rates,
            children: children,
        }
    }
    fn union(self: &mut Self, child: u32, k: u32) {
        unimplemented!();
    }
    fn min(self: &mut Self) -> u32 {
        let mut m = 10000000000;
        for i in 0..self.n {
            let s = self.rates[self.root(i) as usize];
            if s < m {
                m = s;
            }
        }
        return m;
    }
    fn root(self: &mut Self, idx: usize) -> u32 {}
}

fn main() {
    input!(n: usize, q: usize);
    input!(abs: [(u32, u32); n]);
    input!(cds: [(usize, u32); q]);

    let mut rates = vec![0; n];
    let mut children = vec![0; n];
    for i in 0..n {
        let (r, k) = abs[i];
        rates[i] = r;
        children[i] = k;
    }

    let u = U::new(n, rates, children);

    println!("{:?}", abs);
    println!("{:?}", cds);
}
