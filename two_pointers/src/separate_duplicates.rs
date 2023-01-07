fn remove_duplicates(arr: &mut [u8]) -> usize {
    let mut next_non_duplicate: usize = 1;

    for i in 0..arr.len() {
        if arr[next_non_duplicate - 1] != arr[i] {
            arr[next_non_duplicate] = arr[i];
            next_non_duplicate += 1;
        }
    }
    next_non_duplicate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut arr: [u8; 4] = [2, 2, 2, 11];

        let expected: usize = 2;

        let got = remove_duplicates(&mut arr);

        assert_eq!(got, expected);
    }
}
