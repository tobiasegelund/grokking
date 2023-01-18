fn find_duplicate(nums: &mut [usize]) -> usize {
    let mut i = 0;
    while i < nums.len() {
        if nums[i] != i + 1 {
            let j = nums[i] - 1;
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                return nums[i];
            }
        } else {
            i += 1;
        }
    }
    9999
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_numbers() {
        let mut arr = [1, 4, 4, 3, 2];

        let expected = 4;

        let got = find_duplicate(&mut arr);

        assert_eq!(got, expected);
    }
}
