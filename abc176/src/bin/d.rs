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

    let mut d = vec![-1; w * h];
    let mut q = std::collections::BinaryHeap::new();
    q.push((0, sx, sy));

    while let Some((cnt, x, y)) = q.pop() {
        let cnt = -cnt;
        if x == gx && y == gy {
            println!("{}", cnt);
            return;
        }
        if d[index_of(x, y)] != -1 {
            continue;
        }
        d[index_of(x, y)] = cnt;

        for (xx, yy) in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if in_maze_p(xx, yy) {
                if !wall_p(xx, yy) {
                    q.push((-cnt, xx, yy));
                }
            }
        }

        for xd in -2..3 {
            for yd in -2..3 {
                let xx = x + xd;
                let yy = y + yd;
                if in_maze_p(xx, yy) {
                    if !wall_p(xx, yy) {
                        q.push((-cnt - 1, xx, yy));
                    }
                }
            }
        }
    }
    println!("{}", -1);
}
