use proconio::input;

fn main() {
    input!(n: usize);
    let mut tbl: [[i32; 9]; 9] = [[0; 9]; 9];

    for i in 0..n {
        let mut m = i + 1;
        if m % 10 == 0 {
            // 末尾ゼロは数えなくてよい
            continue;
        }
        let lst = m % 10;
        while m >= 10 {
            m = m / 10;
        }
        let fst = m;
        tbl[fst - 1][lst - 1] += 1;
    }

    let mut ans = 0;
    for a in 0..9 {
        for b in 0..9 {
            ans += tbl[a][b] * tbl[b][a];
        }
    }

    println!("{}", ans);
}
