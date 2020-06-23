use proconio::input;

fn binary_search<T, F>(l: T, r: T, f: F) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
    F: Fn(T) -> bool,
{
    let mut l = l;
    let mut r = r;

    loop {
        let idx = (l + r) / T::from(2u8);
        // idx == r is required when l > r.
        if idx == l || idx == r {
            break;
        }
        if f(idx) {
            l = idx;
        } else {
            r = idx;
        }
    }

    l
}
#[cfg(test)]
mod tests {
    use super::binary_search;
    #[test]
    fn it_works() {
        let arr = &vec![1, 2, 3, 4];
        assert_eq!(binary_search(0, arr.len(), |i| arr[i] < 3), 1);
        assert_eq!(binary_search(0, arr.len(), |i| arr[i] < 100), 3);
        assert_eq!(binary_search(0, arr.len(), |i| arr[i] < -1), 0);
        assert_eq!(binary_search(1, 3, |i| arr[i] < 3), 1);
        assert_eq!(binary_search(1, 3, |i| arr[i] < 100), 2);
        assert_eq!(binary_search(1, 3, |i| arr[i] < -1), 1);
        assert_eq!(binary_search(4, -1, |i| arr[i as usize] < 3), 0);
    }
}

fn main() {
    input!(n: usize, k: usize);
    input!(mut aa: [i64; n]);

    aa.sort();

    let lteq_cnt = |m: i64| -> usize {
        let mut cnt = 0;
        for i in 0..aa.len() {
            let a = aa[i];
            if a == 0 {
                if m >= 0 {
                    cnt += n - i - 1;
                }
            } else if a > 0 {
                // Note that l must be i. Not i + 1.
                // When a[i+1] does not match the condition, we can safely ignore it by making j == i.
                let j = binary_search(i, n, |j| a * aa[j] <= m);
                cnt += j - i;
            } else {
                // Same here. n - 1 does not work.
                let j = binary_search(n, i, |j| a * aa[j] <= m);
                cnt += n - j;
            }
        }
        cnt
    };
    let ans = binary_search(
        -1_000_000_000_000_000_001,
        1_000_000_000_000_000_001,
        |m: i64| lteq_cnt(m) < k,
    ) + 1;
    println!("{}", ans);
}
