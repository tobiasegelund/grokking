fn find_missing_numbers(nums: &mut [usize]) -> Vec<usize> {
    let mut i = 0;
    let n = nums.len();
    let mut missing_numbers = Vec::new();

    while i < n {
        let j = nums[i] - 1;

        if nums[i] != nums[j] {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }

    for (idx, val) in nums.iter().enumerate() {
        if idx + 1 != *val {
            missing_numbers.push(idx + 1);
        }
    }
    missing_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_numbers() {
        let mut arr = [2, 3, 1, 8, 2, 3, 5, 1];

        let expected = vec![4, 6, 7];

        let got = find_missing_numbers(&mut arr);

        assert_eq!(got, expected);
    }
}
