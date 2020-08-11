fn main() {
    use proconio::input;
    use proconio::marker::Usize1;
    input!(n: u32, m: usize, sc: [(Usize1, char); m]);
    let l = if n == 1 { 0 } else { 10u32.pow(n - 1) };
    let r = 10u32.pow(n);
    let ok_p = |n: u32| -> bool {
        let st: Vec<char> = n.to_string().chars().collect();
        for (s, c) in sc.iter() {
            if st[*s] != *c {
                return false;
            }
        }
        return true;
    };
    let mut tmp = l;
    while tmp < r {
        if ok_p(tmp) {
            break;
        }
        tmp += 1;
    }

    if tmp == r {
        println!("{}", -1);
    } else {
        println!("{}", tmp);
    }
}
