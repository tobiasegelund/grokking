fn triplet_with_samller_sum(arr: &mut [i32], target: i32) -> i32 {
    arr.sort();
    let mut count: i32 = 0;

    for i in 0..arr.len() - 2 {
        count += search_pair(arr, target - arr[i], i)
    }

    count
}

fn search_pair(arr: &[i32], target_sum: i32, first: usize) -> i32 {
    let mut count: i32 = 0;
    let mut left = first + 1;
    let mut right = arr.len() - 1;

    while left < right {
        if arr[left] + arr[right] < target_sum {
            count += (right - left) as i32;
            left += 1;
        } else {
            right -= 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triplet_with_samller_sum() {
        let mut arr: [i32; 4] = [-1, 0, 2, 3];
        let expected = 2;
        let got = triplet_with_samller_sum(&mut arr, 3);

        assert_eq!(expected, got);
    }
}
