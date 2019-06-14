
mod bubble_sort;
use bubble_sort::*;

mod selection_sort;
use selection_sort::*;

mod insertion_sort;
use insertion_sort::*;

mod quick_sort;
use quick_sort::*;

mod swap;

pub trait Sorter<T>
    where T: Ord
 {
    fn sort(&self, l: &mut Vec<T>);
}

pub enum Sorters {
    Bubble,
    Selection,
    Insertion,
    Quick,
}

static BUBBLE_SORTER: BubbleSorter = BubbleSorter { };
static SELECTION_SORTER: SelectionSorter = SelectionSorter { };
static INSERTION_SORTER: InsertionSorter = InsertionSorter { };
static QUICK_SORTER: QuickSorter = QuickSorter { };

impl Sorters {

    pub fn new<'a, T: Ord>(t: Sorters) -> &'a Sorter<T> {
        match t {
            Sorters::Bubble => &BUBBLE_SORTER,
            Sorters::Selection => &SELECTION_SORTER,
            Sorters::Insertion => &INSERTION_SORTER,
            Sorters::Quick => &QUICK_SORTER,
        }
    } 

}