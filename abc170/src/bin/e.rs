use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeMap;

// https://atcoder.jp/contests/abc170/submissions/14332582

const K: usize = 200_000; // 2 * 10^5

fn main() {
    input!(n: usize, q: usize);
    input!(abs: [(u32, Usize1); n]);
    input!(cds: [(Usize1, Usize1); q]);

    // 各幼稚園毎のBTreeMap の Vec。各木は score => 人数
    let mut mm = vec![BTreeMap::<u32, usize>::new(); K];
    // score => cnt
    let mut maxs = BTreeMap::<u32, usize>::new();

    let rates: Vec<u32> = abs.iter().map(|(a, _)| *a).collect();
    let mut children = vec![0; n];

    for (i, (a, b)) in abs.into_iter().enumerate() {
        *mm[b].entry(a).or_default() += 1;
        children[i] = b;
    }

    for m in mm.iter() {
        if !m.is_empty() {
            let (max, _) = m.iter().last().unwrap();
            *maxs.entry(*max).or_default() += 1;
        }
    }

    for (c, d) in cds {
        // children c goes to d
        let src = children[c];
        let dst = d;

        {
            // src 側の操作
            // とりあえず現在の最大値を最大値リストから抜く
            // → 移動させたやつを引く
            // → 最大値を最大値リストに足す
            let before_max_children_score = *mm[src].iter().last().unwrap().0;
            let r = maxs.entry(before_max_children_score).or_default();

            *r -= 1;
            if *r == 0 {
                maxs.remove(&before_max_children_score);
            }

            let r = mm[src].entry(rates[c]).or_default();
            *r -= 1;
            if *r == 0 {
                mm[src].remove(&rates[c]);
            }

            if !mm[src].is_empty() {
                let after_max_children_score = *mm[src].iter().last().unwrap().0;
                *maxs.entry(after_max_children_score).or_default() += 1;
            }
        }

        children[c] = dst;
        {
            // dst 側の操作
            // とりあえず現在の最大値を最大値リストから抜く
            // → 移動させたやつを足す
            // → 最大値を最大値リストに足す
            if !mm[dst].is_empty() {
                let before_max_children_score = *mm[dst].iter().last().unwrap().0;
                let r = maxs.entry(before_max_children_score).or_default();

                *r -= 1;
                if *r == 0 {
                    maxs.remove(&before_max_children_score);
                }
            }

            let r = mm[dst].entry(rates[c]).or_default();
            *r += 1;

            let after_max_children_score = *mm[dst].iter().last().unwrap().0;
            *maxs.entry(after_max_children_score).or_default() += 1;
        }

        let ans = *maxs.iter().next().unwrap().0;
        println!("{}", ans);
    }
}
