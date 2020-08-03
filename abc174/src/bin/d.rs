fn main() {
    use proconio::input;
    input!(_: usize);
    input!(cs: proconio::marker::Chars);

    let rc = cs.iter().filter(|&c| *c == 'R').count();

    let left_r = cs.iter().take(rc).filter(|&c| *c == 'R').count();

    let tmp = rc - left_r;
    println!("{}", tmp);
}
