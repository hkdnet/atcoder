use proconio::input;

fn main() {
    input!(n: usize);
    input!(qq: [(u64, u64); n]);
    let mut aa = vec![0; n];
    let mut bb = vec![0; n];
    for (i, (a, b)) in qq.iter().enumerate() {
        aa[i] = *a;
        bb[i] = *b;
    }
    aa.sort();
    bb.sort();

    let ans = if n % 2 == 0 {
        let idx = n / 2 - 1;
        let a = (aa[idx] + aa[idx + 1]) * 5;
        let b = (bb[idx] + bb[idx + 1]) * 5;
        (b - a) / 5 + 1
    } else {
        let idx = n / 2;
        let a = aa[idx];
        let b = bb[idx];
        b - a + 1
    };

    println!("{}", ans);
}
