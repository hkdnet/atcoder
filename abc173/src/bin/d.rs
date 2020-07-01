fn main() {
    use proconio::input;
    input!(n: usize);
    input!(mut aa: [u64; n]);

    aa.sort();
    aa.reverse();

    let mut ans = aa[0];
    let mut tmp = n - 2;
    let mut idx = 1;
    while tmp > 0 {
        ans += aa[idx];
        tmp -= 1;
        if tmp == 0 {
            break;
        }
        ans += aa[idx];
        tmp -= 1;
        idx += 1;
    }

    println!("{}", ans);
}
