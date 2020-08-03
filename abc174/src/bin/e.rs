fn binary_search<T, F>(l: T, r: T, f: F) -> T
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

    input!(n: usize, k: u64);
    input!(aa: [u64; n]);

    let mut bb = Vec::with_capacity(n);
    let mut max = aa[0];
    for a in aa {
        let a = a * 2;
        bb.push(a);
        if a > max {
            max = a;
        }
    }

    let tt = binary_search(max, 0, |t| {
        // println!("t = {} ---", t);
        let mut cnt = 0;
        for &b in bb.iter() {
            if b % t == 0 {
                cnt += b / t - 1;
            } else {
                cnt += b / t;
            }
            if cnt > k {
                return false;
            }
            // println!("cnt = {}", cnt);
        }
        true
    });

    if tt % 2 == 0 {
        println!("{}", tt / 2);
    } else {
        println!("{}", (tt / 2) + 1);
    }
}
