fn cycle_sort_nums(arr: &mut [usize]) {
    let mut i = 0;

    while i < arr.len() {
        let j = arr[i] - 1;

        if arr[i] != arr[j] {
            arr.swap(i, j);
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_sort_nums() {
        let mut arr = [3, 2, 4, 1];

        let expected = [1, 2, 3, 4];

        cycle_sort_nums(&mut arr);

        assert_eq!(arr, expected);
    }
}
