
pub fn swap<T: Ord>(l: &mut Vec<T>, i: usize, j: usize) {
    /*
    if i == j {
        return;
    }

    let (i, j) = if i > j { (j, i) } else { (i, j) };
    let temp_j = l.remove(j);
    let temp_i = l.remove(i);
    l.insert(i, temp_j);
    l.insert(j, temp_i);
    */

    l.swap(i, j);
}