fn main() {
    use proconio::input;
    input!(s: proconio::marker::Chars, t: proconio::marker::Chars);

    let l = s.len();
    let mut ans = 0;
    for i in 0..l {
        if s[i] != t[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
