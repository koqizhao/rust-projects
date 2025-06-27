use std::cmp::Ordering;

use super::swap::*;

pub struct QuickSorter {}

impl<T: Ord> super::Sorter<T> for QuickSorter {
    fn sort(&self, l: &mut Vec<T>) {
        Self::sort(l, 0, l.len());
    }
}

impl QuickSorter {
    pub fn sort<T: Ord>(l: &mut Vec<T>, start: usize, end: usize) {
        if start >= end {
            return;
        }

        let mut pivot = (start + end) / 2;
        let (mut i, mut j) = (start, end - 1);
        while i < j {
            while l[i].cmp(&l[pivot]) != Ordering::Greater && i < pivot {
                i += 1;
            }

            swap(l, i, pivot);
            pivot = i;

            while l[j].cmp(&l[pivot]) != Ordering::Less && j > pivot {
                j -= 1;
            }

            swap(l, j, pivot);
            pivot = j;
        }

        Self::sort(l, start, pivot);
        Self::sort(l, pivot + 1, end);
    }
}
