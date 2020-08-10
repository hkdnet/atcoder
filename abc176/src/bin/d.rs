use std::collections::VecDeque;

fn main() {
    use proconio::input;
    use proconio::marker::Isize1;
    input!(h: usize, w: usize);
    let hi = h as isize;
    let wi = w as isize;
    input!(sx: Isize1, sy: Isize1);
    input!(gx: Isize1, gy: Isize1);
    input!(cells: [proconio::marker::Chars; h]);
    let index_of = |x: isize, y: isize| -> usize { (x + y * hi) as usize };
    let in_maze_p = |x: isize, y: isize| -> bool { 0 <= x && x < hi && 0 <= y && y < wi };
    let wall_p = |x: isize, y: isize| -> bool { cells[x as usize][y as usize] == '#' };

    let mut used = vec![false; h * w];
    let mut q: VecDeque<(isize, isize, u32)> = VecDeque::new();

    q.push_back((sx, sy, 0));
    while let Some((x, y, cnt)) = q.pop_front() {
        if used[index_of(x, y)] {
            continue;
        }
        if x == gx && y == gy {
            println!("{}", cnt);
            return;
        }
        used[index_of(x, y)] = true;

        for (xx, yy) in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if in_maze_p(xx, yy) {
                if !wall_p(xx, yy) {
                    if !used[index_of(xx, yy)] {
                        q.push_front((xx, yy, cnt));
                    }
                }
            }
        }

        for xd in -2..3 {
            for yd in -2..3 {
                let xx = x + xd;
                let yy = y + yd;
                if in_maze_p(xx, yy) {
                    if !wall_p(xx, yy) {
                        q.push_back((xx, yy, cnt + 1));
                    }
                }
            }
        }
    }
    println!("{}", -1);
}
