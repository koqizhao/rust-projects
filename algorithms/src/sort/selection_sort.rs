use std::cmp::Ordering;

use super::swap::*;

pub struct SelectionSorter {

}

impl<T: Ord> super::Sorter<T> for SelectionSorter {

    fn sort(&self, l: &mut Vec<T>) {
        Self::sort(l);
    }

}

impl SelectionSorter {

    pub fn sort<T: Ord>(l: &mut Vec<T>) {
        if l.len() <= 1 {
            return;
        }

        for i in 0..(l.len() - 1) {
            for j in (i + 1)..l.len() {
                if l[j].cmp(&l[i]) == Ordering::Less {
                    swap(l, i, j);
                }
            }
        }
    }

}
