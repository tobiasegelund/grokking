fn maximum_sum_subarray(k: usize, arr: &[i32]) -> i32 {
    let mut maximum: i32 = 0;
    let mut window_start: usize = 0;
    let mut sum: i32 = 0;

    for window_end in 0..arr.len() {
        sum += arr[window_end];

        if window_end >= window_start + k - 1 {
            if sum > maximum {
                maximum = sum;
            }
            sum -= arr[window_start];
            window_start += 1;
        }
    }

    maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_sum_subarray() {
        let arr: [i32; 6] = [5, 3, 5, 8, 3, 3];

        let expected: i32 = 8 + 5 + 3;

        let got = maximum_sum_subarray(3, &arr);

        assert_eq!(expected, got);
    }
}
