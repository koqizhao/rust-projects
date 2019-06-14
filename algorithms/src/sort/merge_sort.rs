use std::cmp::Ordering;

pub struct MergeSorter {

}

impl<T: Ord> super::Sorter<T> for MergeSorter {

    fn sort(&self, l: &mut Vec<T>) {
        Self::sort(l, 0, l.len());
    }

}

impl MergeSorter {

    fn sort<T: Ord>(l: &mut Vec<T>, start: usize, end: usize) {
        if end - start <= 1 {
            return;
        }

        let mid = (start + end) / 2;
        Self::sort(l, start, mid);
        Self::sort(l, mid, end);

        let mut temp = Vec::<T>::new();
        let (i, mut j, mut mid, mut end) = (start, mid, mid, end);
        while i < mid && j < end {
            if l[i].cmp(&l[j]) != Ordering::Greater {
                temp.push(l.remove(i));
                mid -= 1;
                j -= 1;
            } else {
                temp.push(l.remove(j));
            }

            end -= 1;
        }

        while i < mid {
            temp.push(l.remove(i));
            mid -= 1;
        }

        while j < end {
            temp.push(l.remove(j));
            end -= 1;
        }

        for k in 0..temp.len() {
            l.insert(start + k, temp.remove(0));
        }
    }

}