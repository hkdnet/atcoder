use proconio::input;
struct S {
    w: usize,
    k: usize,

    acc: Vec<Vec<usize>>, // h, w
}
impl S {
    fn new(h: usize, w: usize, k: usize, ss: Vec<String>) -> Self {
        let mut acc = vec![vec![0; w + 1]; h + 1];
        let cs: Vec<Vec<u8>> = ss.iter().map(|e| e.bytes().collect()).collect();
        for i in 0..w {
            for j in 0..h {
                let c = cs[j][i];
                let tmp = acc[j][i + 1] + acc[j + 1][i] - acc[j][i];
                if c == b'1' {
                    acc[j + 1][i + 1] = tmp + 1;
                } else {
                    acc[j + 1][i + 1] = tmp;
                }
            }
        }
        S {
            w: w,
            k: k,
            acc: acc,
        }
    }
    fn divide_w(self: &Self, hs: Vec<usize>) -> Option<usize> {
        let mut ans = 0;
        let mut last_div = 0;
        loop {
            let mut tmp = last_div;
            'wloop: loop {
                let mut h = 0;
                for &hh in hs.iter() {
                    let delta = (self.acc[hh][tmp] - self.acc[hh][last_div])
                        - (self.acc[h][tmp] - self.acc[h][last_div]);
                    // println!("delta of ({}-{}, {}-{}) = {}", h, hh, last_div, tmp, delta);
                    if delta > self.k {
                        break 'wloop;
                    }
                    h = hh;
                }
                tmp += 1;
                if tmp > self.w {
                    break;
                }
            }
            if tmp == last_div {
                return None;
            }
            let div = tmp - 1;
            // println!("divide at {}, {}-{}", div, last_div, div);
            if div < self.w {
                ans += 1;
            }
            last_div = div;
            if last_div >= self.w {
                break;
            }
        }
        Some(ans)
    }
}

fn main() {
    // 1 <= h <= 10
    input!(h: usize, w: usize, k: usize);
    input!(ss: [String; h]);

    let mut ans = None;
    let s = S::new(h, w, k, ss);

    for i in 0..2i32.pow((h - 1) as u32) {
        let mut hs = vec![];
        for t in 0..h {
            let tt = 2i32.pow(t as u32);
            if (i & tt) == tt {
                hs.push(t + 1);
            }
        }
        hs.push(h);
        // println!("{:?}", hs);
        let len = hs.len();
        match s.divide_w(hs) {
            Some(tmp) => {
                let new_ans = tmp + len - 1;
                match ans {
                    Some(e) => {
                        if e > new_ans {
                            ans = Some(new_ans);
                        }
                    }
                    None => {
                        ans = Some(new_ans);
                    }
                }
            }
            None => {} // do nothing
        };
    }
    let ans = ans.unwrap();
    println!("{}", ans);
}
