use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;
use std::collections::VecDeque;

const WALL: char = '@';

fn main() {
    input!(h: usize, w: usize, k: usize);
    input!(x1: Usize1, y1: Usize1, x2: Usize1, y2: Usize1);
    input!(dots: [proconio::marker::Chars; h]);

    let mut rows = vec![BTreeSet::new(); h];
    let mut cols = vec![BTreeSet::new(); w];

    for x in 0..h {
        for y in 0..w {
            rows[x].insert(y);
            cols[y].insert(x);
        }
    }

    let mut cnts = vec![vec![-1; w]; h];
    let mut q = VecDeque::new();

    q.push_back((x1, y1));
    cnts[x1][y1] = 0;
    rows[x1].remove(&y1);
    cols[y1].remove(&x1);

    while let Some((x, y)) = q.pop_front() {
        let cur = cnts[x][y];
        // x + k
        {
            let mut arr = vec![];
            for &nx in cols[y].range(x + 1..) {
                if nx - x > k || dots[nx][y] == WALL {
                    break;
                }
                cnts[nx][y] = cur + 1;
                q.push_back((nx, y));
                arr.push(nx);
            }
            for nx in arr {
                cols[y].remove(&nx);
                rows[nx].remove(&y);
            }
        }
        // x - k
        {
            let mut arr = vec![];
            for &nx in cols[y].range(..x).rev() {
                if x - nx > k || dots[nx][y] == WALL {
                    break;
                }
                cnts[nx][y] = cur + 1;
                q.push_back((nx, y));
                arr.push(nx);
            }
            for nx in arr {
                cols[y].remove(&nx);
                rows[nx].remove(&y);
            }
        }
        // y + k
        {
            let mut arr = vec![];
            for &ny in rows[x].range(y + 1..) {
                if ny - y > k || dots[x][ny] == WALL {
                    break;
                }
                cnts[x][ny] = cur + 1;
                q.push_back((x, ny));
                arr.push(ny);
            }
            for ny in arr {
                cols[ny].remove(&x);
                rows[x].remove(&ny);
            }
        }
        // y - k
        {
            let mut arr = vec![];
            for &ny in rows[x].range(..y).rev() {
                if y - ny > k || dots[x][ny] == WALL {
                    break;
                }
                cnts[x][ny] = cur + 1;
                q.push_back((x, ny));
                arr.push(ny);
            }
            for ny in arr {
                cols[ny].remove(&x);
                rows[x].remove(&ny);
            }
        }
    }

    let ans = cnts[x2][y2];
    println!("{}", ans);
}
