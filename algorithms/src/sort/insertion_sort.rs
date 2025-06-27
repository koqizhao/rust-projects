use std::cmp::Ordering;

pub struct InsertionSorter {}

impl<T: Ord> super::Sorter<T> for InsertionSorter {
    fn sort(&self, l: &mut Vec<T>) {
        Self::sort(l);
    }
}

impl InsertionSorter {
    pub fn sort<T: Ord>(l: &mut Vec<T>) {
        if l.len() <= 1 {
            return;
        }

        for i in 1..l.len() {
            for j in 0..i {
                if l[j].cmp(&l[i]) == Ordering::Greater {
                    let temp = l.remove(i);
                    l.insert(j, temp);
                    break;
                }
            }
        }
    }
}
