fn main() {
    use proconio::input;
    input!(n: usize);
    input!(xx: [i32; n]);

    let loss = |p: i32| -> i32 {
        let mut ret = 0;
        for x in xx.iter() {
            ret += (p - x) * (p - x);
        }
        ret
    };

    let sum = xx.iter().fold(0, |a, e| a + e);
    let m = sum / n as i32;
    let l = std::cmp::min(loss(m), loss(m + 1));
    println!("{}", l);
}
