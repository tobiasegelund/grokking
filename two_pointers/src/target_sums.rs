fn pair_with_targetsum(arr: &[u8], target_sum: u8) -> [usize; 2] {
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;

    while left < right {
        let current_sum = arr[left] + arr[right];

        if current_sum == target_sum {
            return [left, right];
        } else if current_sum < target_sum {
            left += 1;
        } else {
            right -= 1;
        }
    }
    [0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_with_targetsum() {
        let arr: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let expected: [usize; 2] = [0, 4];
        let got: [usize; 2] = pair_with_targetsum(&arr, 6);

        assert_eq!(got, expected);
    }
}
