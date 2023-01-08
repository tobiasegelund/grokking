fn dutch_flag_sort(arr: &mut [u8]) {
    let mut low: usize = 0;
    let mut high = arr.len() - 1;

    let mut i = 0;
    while i <= high {
        if arr[i] == 0 {
            let tmp_low = arr[low];
            arr[low] = arr[i];
            arr[i] = tmp_low;

            i += 1;
            low += 1;
        } else if arr[i] == 1 {
            i += 1;
        } else {
            let tmp_high = arr[high];
            arr[high] = arr[i];
            arr[i] = tmp_high;

            high -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dutch_flag_sort() {
        let mut arr: [u8; 6] = [2, 2, 0, 1, 2, 0];

        let expected: [u8; 6] = [0, 0, 1, 2, 2, 2];

        dutch_flag_sort(&mut arr);

        assert_eq!(arr, expected);
    }
}
