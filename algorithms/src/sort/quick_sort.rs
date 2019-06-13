
pub fn quick_sort(l: &mut Vec<i32>) {
    _quick_sort(l, 0, l.len());
}

fn _quick_sort(l: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mut pivot = (start + end) / 2;
    let v = l[pivot];
    let (mut i, mut j) = (start, end - 1);
    while i < j {
        while l[i] <= v && i < pivot {
            i += 1;
        }

        l[pivot] = l[i];
        pivot = i;

        while l[j] >= v && j > pivot {
            j -= 1;
        }

        l[pivot] = l[j];
        pivot = j;
    }

    l[pivot] = v;

    _quick_sort(l, start, pivot);
    _quick_sort(l, pivot + 1, end);
}
