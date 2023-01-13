fn maxmium_number_of_fruits(arr: &[char], k: Option<usize>) -> usize {
    // Rusty way of default args
    let k = k.unwrap_or(2);

    let mut window_start = 0;
    let mut max_length = 0;
    let mut fruit_frequency = std::collections::HashMap::new();

    for window_end in 0..arr.len() {
        let right_char = arr[window_end];
        *fruit_frequency.entry(right_char).or_insert(0) += 1;

        while fruit_frequency.len() > k {
            let left_char = arr[window_start];
            *fruit_frequency.get_mut(&left_char).unwrap() -= 1;
            if fruit_frequency[&left_char] == 0 {
                fruit_frequency.remove(&left_char);
            }
            window_start += 1;
        }

        max_length = std::cmp::max(max_length, window_end - window_start + 1);
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maxmium_number_of_fruits() {
        let arr = ['A', 'B', 'C', 'A', 'C'];

        let expected = 3;

        let got = maxmium_number_of_fruits(&arr, Some(2));

        assert_eq!(expected, got);
    }
}
