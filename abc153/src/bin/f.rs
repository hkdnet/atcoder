use proconio::input;

fn main() {
    // 結局左のやつを倒せるように順番にやっていけばよい
    input!(n: usize, d: usize, a: usize);
    input!(mut hs: [(usize, usize); n]);
    hs.sort();

    let mut idx = 0;
    let mut min = 0;

    // for each i calculate the max of j where xj - xi <= 2* d

    'bomb: loop {
        'find_left: loop {
            let (_, v) = hs[idx];
            if v > 0 {
                break 'find_left;
            }
            idx += 1;
            if idx >= n {
                break 'bomb;
            }
        }

        let (left_x, left_h) = hs[idx];

        let bombs = if left_h % a == 0 {
            left_h / a
        } else {
            left_h / a + 1
        };
        min += bombs;
        let damage = a * bombs;

        let mut tmp_idx = idx;
        let target = left_x + d;
        let right = target + d; // 端点含む

        loop {
            if tmp_idx >= n {
                break;
            }
            let (i, h) = hs[tmp_idx];
            if i > right {
                break;
            }

            if h <= damage {
                hs[tmp_idx] = (i, 0);
            } else {
                hs[tmp_idx] = (i, h - damage);
            }

            tmp_idx += 1;
        }
    }
    println!("{}", min);
}
