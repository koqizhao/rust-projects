pub mod sort;

#[cfg(test)]
mod tests {

    use std::fmt::*;
    use super::sort::*;

    #[test]
    fn test_bubble_sort() {
        let sorter = Sorters::new::<i32>(Sorters::BubbleSort);
        test_sort_i32(sorter);
    }

    #[test]
    fn test_selection_sort() {
        let sorter = Sorters::new::<i32>(Sorters::SelectionSort);
        test_sort_i32(sorter);
    }

    #[test]
    fn test_insertion_sort() {
        let sorter = Sorters::new::<i32>(Sorters::InsertionSort);
        test_sort_i32(sorter);
    }

    #[test]
    fn test_quick_sort() {
        let sorter = Sorters::new::<i32>(Sorters::QuickSort);
        test_sort_i32(sorter);
    }

    #[test]
    fn test_merge_sort() {
        let sorter = Sorters::new::<i32>(Sorters::MergeSort);
        test_sort_i32(sorter);
    }

    fn test_sort_i32(sorter: &Sorter<i32>) {
        let mut l = Vec::new();
        l.push(1);
        l[0] = 2;
        l.push(3);
        l.push(1);

        let expected = vec![1, 2, 3];
        test_sort(sorter, &mut l, &expected);
    }

    fn test_sort<T: Ord + Display + Debug>(sorter: &Sorter<T>, l: &mut Vec<T>, expected: &Vec<T>) {
        print_vec(l.as_ref());
        sorter.sort(l);
        print_vec(l.as_ref());
        assert_eq!(l, expected);
    }

    fn print_vec<T: Display>(l: &Vec<T>) {
        for e in l {
            println!("{} ", e);
        }
    }
}
