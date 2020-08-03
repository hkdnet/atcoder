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
    use std::collections::BinaryHeap;

    input!(n: usize, k: u64);
    input!(aa: [u64; n]);

    let mut h: BinaryHeap<u64> = BinaryHeap::new();
    for a in aa {
        // 0.5単位でしか見ないので2倍しておく
        h.push(a * 2);
    }

    let max = *h.peek().unwrap();

    let tt = binary_search(max, 0, |t| {
        // println!("t = {} ---", t);
        let mut h = h.clone();
        for _ in 0..k {
            match h.pop() {
                Some(e) => {
                    if e <= t {
                        return true;
                    }
                    h.push(e - t);

                    // println!("{:?}", h);
                }
                None => {
                    return true;
                }
            }
        }
        match h.peek() {
            Some(&e) => e <= t,
            None => true,
        }
    });

    // println!("tt = {}", tt);

    if tt % 2 == 0 {
        println!("{}", tt / 2);
    } else {
        println!("{}", (tt / 2) + 1);
    }
}
