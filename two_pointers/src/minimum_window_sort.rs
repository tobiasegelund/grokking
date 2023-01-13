fn minimum_window_sort(arr: &[i32]) -> i32 {
    let mut counter = 0;
    let mut low = 0;
    let mut high = arr.len() - 1;

    while (low < arr.len() - 1) && (arr[low] <= arr[low + 1]) {
        low += 1;
    }

    if low == arr.len() - 1 {
        return 0;
    }

    while (high > 0) && (arr[high] >= arr[high - 1]) {
        high -= 1;
    }

    let mut subarray_max = -9999;
    let mut subarray_min = 9999;
    for i in low..high {
        if arr[i] > subarray_max {
            subarray_max = arr[i];
        }
        if arr[i] < subarray_min {
            subarray_min = arr[i];
        }
    }

    while (low > 0) && (arr[low - 1] > subarray_min) {
        low -= 1;
    }

    while (high < arr.len() - 1) && (arr[high + 1] < subarray_max) {
        high += 1;
    }

    (high - low + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_window_sort() {
        let arr = [1, 3, 2, 0, -1, 7, 10];

        let expected = 5;

        let got = minimum_window_sort(&arr);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_minimum_window_sorted_array() {
        let arr = [1, 2, 3];

        let expected = 0;

        let got = minimum_window_sort(&arr);

        assert_eq!(expected, got);
    }
}
