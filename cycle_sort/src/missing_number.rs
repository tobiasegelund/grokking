fn find_missing_number(nums: &mut [usize]) -> usize {
    let mut i = 0;
    let n = nums.len();

    while i < n {
        let j = nums[i];

        if nums[i] < n && nums[i] != nums[j] {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }

    for i in 0..n {
        if i != nums[i] {
            return i;
        }
    }

    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_numbers() {
        let mut arr = [4, 0, 3, 1];

        let expected = 2;

        let got = find_missing_number(&mut arr);

        assert_eq!(got, expected);
    }
}
