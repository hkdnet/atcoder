fn main() {
    use proconio::input;
    input!(n: usize);
    input!(mut ll: [usize; n]);

    ll.sort();

    let mut cnt = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if ll[i] == ll[j] {
                    continue;
                }
                if ll[j] == ll[k] {
                    continue;
                }
                if ll[i] + ll[j] > ll[k] {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
