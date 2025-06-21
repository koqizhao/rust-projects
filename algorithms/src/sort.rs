mod bubble_sort;
use bubble_sort::*;

mod selection_sort;
use selection_sort::*;

mod insertion_sort;
use insertion_sort::*;

mod quick_sort;
use quick_sort::*;

mod merge_sort;
use merge_sort::*;

mod swap;

pub trait Sorter<T>
where
    T: Ord,
{
    fn sort(&self, l: &mut Vec<T>);
}

pub enum Sorters {
    BubbleSort,
    SelectionSort,
    InsertionSort,
    QuickSort,
    MergeSort,
}

static BUBBLE_SORTER: BubbleSorter = BubbleSorter {};
static SELECTION_SORTER: SelectionSorter = SelectionSorter {};
static INSERTION_SORTER: InsertionSorter = InsertionSorter {};
static QUICK_SORTER: QuickSorter = QuickSorter {};
static MERGE_SORTER: MergeSorter = MergeSorter {};

impl Sorters {
    pub fn new<'a, T: Ord>(t: Sorters) -> &'a dyn Sorter<T> {
        match t {
            Sorters::BubbleSort => &BUBBLE_SORTER,
            Sorters::SelectionSort => &SELECTION_SORTER,
            Sorters::InsertionSort => &INSERTION_SORTER,
            Sorters::QuickSort => &QUICK_SORTER,
            Sorters::MergeSort => &MERGE_SORTER,
        }
    }
}
