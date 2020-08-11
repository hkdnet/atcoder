fn main() {
    use proconio::input;
    input!(aa: [u32; 9]);
    input!(n: usize, bb: [u32; n]);
    let mut used = vec![false; 9];

    for b in bb {
        if let Some(idx) = aa.iter().position(|&a| a == b) {
            used[idx] = true;
        }
    }
    let lines = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
    for ii in lines {
        let mut ok = true;
        for i in ii {
            ok = ok && used[i];
        }

        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
