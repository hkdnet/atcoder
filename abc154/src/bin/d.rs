use proconio::input;

fn main() {
    input!(n: usize, k: usize);
    input!(pp: [usize; n]);

    let mut sum = vec![0; n + 1];
    sum[0] = pp[0];
    for i in 0..n {
        sum[i + 1] = sum[i] + pp[i];
    }
    let mut max = 0;
    for i in 0..(n - k + 1) {
        let tmp = sum[i + k] - sum[i];
        if tmp > max {
            max = tmp;
        }
    }

    let tmp = max + k;
    println!("{}", tmp as f32 / 2f32);
}
