use std::collections::BTreeMap;

fn popcount(x: u32) -> u32 {
    if x <= 1 {
        return x;
    }

    let mut x = x;
    let mut cnt = 0;
    while x != 1 {
        if x % 2 == 1 {
            cnt += 1;
        }
        x /= 2;
    }

    cnt + 1
}

fn exec(x: u32, memo: &BTreeMap<u32, u32>) -> u32 {
    let mut tmp = x;
    let mut num = 0;
    while tmp != 0 {
        if memo.contains_key(&tmp) {
            num += memo.get(&tmp).unwrap();
            break;
        } else {
            let c = popcount(tmp);
            tmp %= c;
            num += 1;
        }
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popcount() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(1), 1);
        assert_eq!(popcount(2), 1);
        assert_eq!(popcount(3), 2);
        assert_eq!(popcount(4), 1);
        assert_eq!(popcount(5), 2);
        assert_eq!(popcount(6), 2);
    }

    #[test]
    fn test_exec() {
        let memo = BTreeMap::new();
        assert_eq!(exec(0, &memo), 0);
        assert_eq!(exec(1, &memo), 1);
        assert_eq!(exec(7, &memo), 2);
    }
}

fn main() {
    use proconio::input;
    input!(n: usize);
    input!(mut cs: proconio::marker::Chars);

    let mut xi = 0;
    for &c in cs.iter() {
        if c == '1' {
            xi += 1;
        }
    }

    if xi == 0 {
        for _ in 0..n {
            println!("{}", 1);
        }
        return;
    } else if xi == 1 {
        let def = if cs.last().unwrap() == &'1' { 0 } else { 1 };

        for &c in cs.iter() {
            if c == '1' {
                println!("{}", 0);
            } else {
                // 常に % 2 される
                println!("{}", def + 1);
            }
        }

        return;
    };

    // 以下 xi >= 2

    let mut x1 = 0;
    let mut x2 = 0;
    let mut tmp1 = 1;
    let mut tmp2 = 1;
    cs.reverse();
    for &c in cs.iter() {
        if c == '1' {
            x1 += tmp1;
            x1 %= xi + 1;
            x2 += tmp2;
            x2 %= xi - 1;
        }

        tmp1 *= 2;
        tmp1 %= xi + 1;
        tmp2 *= 2;
        tmp2 %= xi - 1;
    }

    let mut memo: BTreeMap<u32, u32> = BTreeMap::new();
    memo.insert(0, 0);

    for i in 1..xi + 2 {
        let num = exec(i, &memo);
        memo.insert(i, num);
    }

    // println!("{:?}", memo);

    let mut del1 = 1;
    let mut del2 = 1;
    let mut ans = vec![];
    // println!("{}, {}", x1, x2);
    for &c in cs.iter() {
        let x = if c == '0' {
            // 1 にするので del1 側を使う
            (x1 + del1) % (xi + 1)
        } else {
            let tmp = if x2 < del2 {
                x2 + (xi - 1) - del2
            } else {
                x2 - del2
            };
            tmp % (xi - 1)
        };

        // println!("c = {} -> x = {}", c, x);

        let a = memo.get(&x).unwrap();
        ans.push(a + 1);

        del1 *= 2;
        del1 %= xi + 1;
        del2 *= 2;
        del2 %= xi - 1;
    }
    let l = ans.len();
    for i in 0..l {
        let idx = l - i - 1;
        println!("{}", ans[idx]);
    }
}
