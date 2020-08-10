use std::collections::BTreeSet;
use std::collections::BinaryHeap;

fn main() {
    use proconio::input;
    use proconio::marker::Usize1;
    input!(h: usize, w: usize);
    input!(m: usize);
    let mut bb = BTreeSet::new();
    for _ in 0..m {
        input!(a: Usize1, b: Usize1);
        bb.insert((a, b));
    }
    let mut h_cnt = vec![0; h];
    let mut w_cnt = vec![0; w];
    for &(x, y) in bb.iter() {
        h_cnt[x] += 1;
        w_cnt[y] += 1;
    }
    let (x_max, hs) = {
        let mut xh = BinaryHeap::new();
        for (x, &cnt) in h_cnt.iter().enumerate() {
            xh.push((cnt, x));
        }
        let mut hs = Vec::new();
        let (x_max, x) = xh.pop().unwrap();
        hs.push(x);
        while let Some((tmp, xx)) = xh.pop() {
            if tmp == x_max {
                hs.push(xx);
            } else {
                break;
            }
        }
        (x_max, hs)
    };
    let (y_max, ws) = {
        let mut yh = BinaryHeap::new();
        for (y, &cnt) in w_cnt.iter().enumerate() {
            yh.push((cnt, y));
        }
        let mut ws = Vec::new();
        let (y_max, y) = yh.pop().unwrap();
        ws.push(y);

        while let Some((tmp, yy)) = yh.pop() {
            if tmp == y_max {
                ws.push(yy);
            } else {
                break;
            }
        }
        (y_max, ws)
    };

    let mut found = false;
    'l: for xx in hs {
        for &yy in ws.iter() {
            if !bb.contains(&(xx, yy)) {
                found = true;
                break 'l;
            }
        }
    }
    let ans = x_max + y_max;

    if found {
        println!("{}", ans);
    } else {
        println!("{}", ans - 1);
    }
}
