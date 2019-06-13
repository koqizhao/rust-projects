use super::swap::*;
use itertools::rev;

pub fn bubble_sort(l: &mut Vec<i32>) {
    for i in rev(1..l.len()) {
        for j in 0..i {
            if l[j] > l[j + 1] {
                swap(l, j, j + 1);
            }
        }
    }
}
