fn main() {
    use proconio::input;
    input!(a: u32, b: u32, c: u32, k: u32);
    let mut b = b;
    let mut c = c;
    let mut cnt = 0;
    while a >= b {
        b *= 2;
        cnt += 1;
    }
    while b >= c {
        c *= 2;
        cnt += 1;
    }

    if cnt <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
