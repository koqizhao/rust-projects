use std::cmp::Ordering;
use itertools::rev;

use super::swap::*;

pub struct BubbleSorter {

}

impl<T: Ord> super::Sorter<T> for BubbleSorter {

    fn sort(&self, l: &mut Vec<T>) {
        Self::sort(l);
    }

}

impl BubbleSorter {

    pub fn sort<T: Ord>(l: &mut Vec<T>) {
        for i in rev(1..l.len()) {
            for j in 0..i {
                if l[j].cmp(&l[j + 1]) == Ordering::Greater {
                    swap(l, j, j + 1);
                }
            }
        }
    }

}
