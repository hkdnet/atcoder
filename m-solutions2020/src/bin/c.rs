fn main() {
    use proconio::input;
    input!(n: usize, k: usize);
    input!(aa: [u32; n]);

    for i in k..n {
        let j = i - k;
        if aa[i] > aa[j] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
