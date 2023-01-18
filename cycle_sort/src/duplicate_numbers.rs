fn find_duplicate_numbers(nums: &mut [usize]) -> Vec<usize> {
    let mut i = 0;
    let mut duplicate_numbers = Vec::new();

    while i < nums.len() {
        if nums[i] != i + 1 {
            let j = nums[i] - 1;
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                duplicate_numbers.push(nums[i]);
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    duplicate_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_numbers() {
        let mut arr = [3, 4, 4, 5, 5];

        let expected = vec![5, 4];

        let got = find_duplicate_numbers(&mut arr);

        assert_eq!(got, expected);
    }
}
