use crate::Interval;

fn conflicting_appointments(arr: &mut Vec<Interval>) -> bool {
    arr.sort_by(|a, b| a.start.cmp(&b.start));

    for i in 0..arr.len() {
        if i == arr.len() - 1 {
            break;
        }

        let left_interval = &arr[i];
        let right_interval = &arr[i + 1];

        if left_interval.end >= right_interval.start {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflicting_appointments() {
        let mut arr = vec![
            Interval::new(1, 4),
            Interval::new(2, 5),
            Interval::new(7, 9),
        ];

        let expected = false;

        let got = conflicting_appointments(&mut arr);
        assert_eq!(expected, got);
    }
}
