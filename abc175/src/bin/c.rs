fn main() {
    use proconio::input;
    input!(mut x: i128, k: i128, d: i128);
    if x < 0 {
        x = -x;
    }

    if x / d > k {
        x -= d * k;
        return;
    }
}
