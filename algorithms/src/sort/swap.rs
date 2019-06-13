
pub fn swap(l: &mut Vec<i32>, i: usize, j: usize) {
    let temp = l[i];
    l[i] = l[j];
    l[j] = temp;
}