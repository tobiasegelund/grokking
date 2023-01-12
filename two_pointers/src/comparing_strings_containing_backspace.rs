fn backspace_compare(str1: &str, str2: &str) -> bool {
    let mut index1: isize = (str1.len() - 1) as isize;
    let mut index2: isize = (str2.len() - 1) as isize;
    while (index1 >= 0) || (index2 >= 0) {
        let i1 = get_next_valid_char_index(str1, index1);
        let i2 = get_next_valid_char_index(str2, index2);
        if i1 < 0 && i2 < 0 {
            return true;
        }
        if i1 < 0 || i2 < 0 {
            return false;
        }
        if str1.chars().nth(i1 as usize) != str2.chars().nth(i2 as usize) {
            return false;
        }
        index1 = i1 - 1;
        index2 = i2 - 1;
    }
    true
}

fn get_next_valid_char_index(str: &str, mut index: isize) -> isize {
    let mut backspace_count = 0;
    while index >= 0 {
        if str.chars().nth(index as usize) == Some('#') {
            backspace_count += 1;
        } else if backspace_count > 0 {
            backspace_count -= 1;
        } else {
            break;
        }
        index -= 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare_true() {
        let str1 = "xy#z";
        let str2 = "xzz#";

        let expected = true;
        let got = backspace_compare(str1, str2);

        assert_eq!(got, expected);
    }

    #[test]
    fn test_backspace_compare_false() {
        let str1 = "xy#z";
        let str2 = "xyz#";

        let expected = false;
        let got = backspace_compare(str1, str2);

        assert_eq!(got, expected);
    }
}
