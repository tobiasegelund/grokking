use crate::longest_substring_w_k_distinct_chars;

fn permutation_in_string(s: &str, pattern: &str) -> bool {
    let mut window_start: usize = 0;
    let mut matched = 0;
    let mut char_frequency = std::collections::HashMap::new();

    for i in 0..pattern.len() {
        let char = s.chars().nth(i).unwrap();
        *char_frequency.entry(char).or_insert(0) += 1;
    }

    for window_end in 0..s.len() {
        let right_char = s.chars().nth(window_end).unwrap();
        if char_frequency.contains_key(&right_char) {
            *char_frequency.get_mut(&right_char).unwrap() -= 1;

            if char_frequency[&right_char] == 0 {
                matched += 1
            }
        }

        if matched == char_frequency.len() {
            return true;
        }

        if window_end >= pattern.len() - 1 {
            let left_char = s.chars().nth(window_start).unwrap();
            window_start += 1;

            if char_frequency.contains_key(&left_char) {
                if char_frequency[&left_char] == 0 {
                    matched -= 1;
                }
                *char_frequency.get_mut(&left_char).unwrap() += 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_in_string() {
        let s = "oidbcaf";

        let pattern = "abc";

        let expected = true;

        let got = permutation_in_string(&s, &pattern);

        assert_eq!(expected, got);
    }
}
