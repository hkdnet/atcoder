use proconio::input;

fn main() {
    input!(h: usize, n: usize);
    input!(ms: [(usize, i128); n]);

    let mut used_powers: Vec<i128> = vec![-1; h + 10000 + 1];
    used_powers[0] = 0;

    for (a, b) in ms {
        for i in 0..h {
            if used_powers[i] < 0 {
                continue;
            }
            let idx = i + a;
            let np = used_powers[i] + b;
            if used_powers[idx] < 0 || np < used_powers[idx] {
                used_powers[idx] = np;
            }
        }
    }
    let mut min = used_powers[h];

    for idx in h..used_powers.len() {
        if used_powers[idx] != -1 && (min == -1 || min > used_powers[idx]) {
            min = used_powers[idx];
        }
    }
    println!("{}", min);
}
