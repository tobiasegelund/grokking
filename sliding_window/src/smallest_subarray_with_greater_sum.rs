fn smallest_subarray_w_greater_sum(arr: &[u8], s: u8) -> u8 {
    let mut length = 0;

    for i in 0..arr.len() {
        for j in i..arr.len() {
            let sum: u8 = arr[i..j].iter().sum();
            if sum >= s {
                if (j - i) < length || length == 0 {
                    length = j - i;
                    break;
                }
            }
        }
    }
    length as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_subarray() {
        let arr = [2, 1, 5, 2, 3, 2];

        let expected = 2;

        let got = smallest_subarray_w_greater_sum(&arr, 7);

        assert_eq!(expected, got);
    }
}
