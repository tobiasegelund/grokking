fn longest_substring_distinct_chars(str1: &str, k: usize) -> usize {
    let mut window_start = 0;
    let mut max_length = 0;
    let mut char_frequency = std::collections::HashMap::new();

    for window_end in 0..str1.len() {
        let right_char = str1.chars().nth(window_end).unwrap();

        if char_frequency.contains_key(&right_char) {
            window_start = std::cmp::max(window_start, char_frequency[&right_char] + 1);
        }

        *char_frequency.entry(right_char).or_insert(window_end) = window_end;
        max_length = std::cmp::max(max_length, window_end - window_start + 1);
    }
    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_longest_substring() {
        let input = "aabccbb";

        let expected: usize = 3;

        let got = longest_substring_distinct_chars(input, 2);

        assert_eq!(expected, got);
    }
}
