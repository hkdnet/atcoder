pub fn binary_search<T, F>(l: T, r: T, f: F) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
    F: Fn(T) -> bool,
{
    let mut l = l;
    let mut r = r;

    loop {
        let idx = (l + r) / T::from(2u8);
        // idx == r is required when l > r.
        if idx == l || idx == r {
            break;
        }
        if f(idx) {
            l = idx;
        } else {
            r = idx;
        }
    }

    l
}

fn main() {
    use proconio::input;
    input!(n: usize, m: usize, k: i128);

    input!(aa: [i128; n]);
    input!(bb: [i128; m]);

    let mut acca = vec![0; n + 1];
    let mut accb = vec![0; m + 1];
    let mut ai = 0;
    for (i, a) in aa.iter().enumerate() {
        acca[i + 1] = acca[i] + a;
        if acca[i + 1] <= k {
            ai = i + 1;
        }
    }
    for i in 0..m {
        accb[i + 1] = accb[i] + bb[i];
    }
    let mut ans = 0;
    let mut bi = 0;

    loop {
        // binsearch?
        loop {
            if bi >= m || acca[ai] + accb[bi + 1] > k {
                break;
            }
            bi += 1;
        }
        ans = std::cmp::max(ai + bi, ans);
        if ai == 0 || bi >= m {
            break;
        }
        ai -= 1;
    }

    println!("{}", ans);
}
