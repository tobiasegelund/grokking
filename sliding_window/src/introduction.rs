fn find_averages_of_subarrays_slow(k: usize, arr: &[i32]) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();

    for i in 0..arr.len() - k + 1 {
        let sum: i32 = arr[i..i + k].iter().sum();
        result.push(sum as f32 / k as f32);
    }

    result
}

fn find_averages_of_subarrays_fast(k: usize, arr: &[i32]) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();
    let mut window_sum: i32 = 0;
    let mut window_start: usize = 0;

    for window_end in 0..arr.len() {
        window_sum += arr[window_end];

        if window_end >= k - 1 {
            result.push(window_sum.clone() as f32 / k as f32);
            window_sum -= arr[window_start];
            window_start += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_averages_of_subarrays_slow() {
        let arr = [1, 3, 2, 6, -1, 4, 1, 8, 2];

        let expected: Vec<f32> = vec![2.2, 2.8, 2.4, 3.6, 2.8];

        let got = find_averages_of_subarrays_slow(5, &arr);
        assert_eq!(got, expected);
    }

    #[test]
    fn test_find_averages_of_subarrays_fast() {
        let arr = [1, 3, 2, 6, -1, 4, 1, 8, 2];

        let expected: Vec<f32> = vec![2.2, 2.8, 2.4, 3.6, 2.8];

        let got = find_averages_of_subarrays_fast(5, &arr);
        assert_eq!(got, expected);
    }
}
