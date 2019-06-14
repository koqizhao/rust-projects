use super::swap::*;

pub fn selection_sort(l: &mut Vec<i32>) {
    if l.len() <= 1 {
        return;
    }

    for i in 0..(l.len() - 1) {
        for j in (i + 1)..l.len() {
            if l[j] < l[i] {
                swap(l, i, j);
            }
        }
    }
}