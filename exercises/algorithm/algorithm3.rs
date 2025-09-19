/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn merge<T: PartialOrd + Clone>(array: &mut [T], left: usize, mid: usize, right: usize) {
    let left_array = array[left..mid].to_vec();
    let right_array = array[mid..right].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < left_array.len() && j < right_array.len() {
        if left_array[i] <= right_array[j] {
            array[k] = left_array[i].clone();
            i += 1;
        } else {
            array[k] = right_array[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left_array.len() {
        array[k] = left_array[i].clone();
        i += 1;
        k += 1;
    }

    while j < right_array.len() {
        array[k] = right_array[j].clone();
        j += 1;
        k += 1;
    }
}

fn merge_sort<T: PartialOrd + Clone>(array: &mut [T], left: usize, right: usize) {
    if left + 1 < right {
        let mid = left + (right - left) / 2;

        merge_sort(array, left, mid);
        merge_sort(array, mid, right);
        merge(array, left, mid, right);
    }
}

pub fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    if array.len() > 1 {
        merge_sort(array, 0, array.len());
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}