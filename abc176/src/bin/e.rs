fn main() {
    use proconio::input;
    input!(h: usize, w: usize);
    input!(m: usize);
    input!(bb: [(usize, usize); m]);
    use std::collections::BTreeMap;
    let mut h_cells: BTreeMap<usize, BTreeMap<usize, bool>> = BTreeMap::new();
    let mut w_cells: BTreeMap<usize, BTreeMap<usize, bool>> = BTreeMap::new();
    for (x, y) in bb {
        h_cells.entry(x).or_insert(BTreeMap::new()).insert(y, true);
        w_cells.entry(y).or_insert(BTreeMap::new()).insert(x, true);
    }
}
