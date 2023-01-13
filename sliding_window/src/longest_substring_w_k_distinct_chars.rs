fn longest_substring_with_k_distinct(str1: &str, k: usize) -> usize {
    let mut window_start = 0;
    let mut max_length = 0;
    let mut char_frequency = std::collections::HashMap::new();

    for window_end in 0..str1.len() {
        let right_char = str1.chars().nth(window_end).unwrap();
        *char_frequency.entry(right_char).or_insert(0) += 1;

        while char_frequency.len() > k {
            let left_char = str1.chars().nth(window_start).unwrap();
            *char_frequency.get_mut(&left_char).unwrap() -= 1;
            if char_frequency[&left_char] == 0 {
                char_frequency.remove(&left_char);
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
    fn test_find_longest_substring() {
        let input = "araaci";

        let expected: usize = 4;

        let got = longest_substring_with_k_distinct(input, 2);

        assert_eq!(expected, got);
    }
}
