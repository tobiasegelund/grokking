use crate::Interval;

fn find_minimum_meeting_rooms(arr: &mut Vec<Interval>) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut minimum_rooms = 1; // At least one room will be needed
    arr.sort_by(|a, b| a.start.cmp(&b.start));

    let mut end = arr[0].end;

    for i in 1..arr.len() {
        let interval = &arr[i];
        if interval.start <= end {
            end = std::cmp::max(interval.end, end);
        } else {
            minimum_rooms += 1;
            end = interval.end;
        }
    }

    minimum_rooms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_minimum_meeting_rooms() {
        let mut arr = vec![
            Interval::new(1, 4),
            Interval::new(2, 5),
            Interval::new(7, 9),
        ];

        let expected = 2;

        let got = find_minimum_meeting_rooms(&mut arr);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_find_minimum_meeting_rooms2() {
        let mut arr = vec![
            Interval::new(4, 5),
            Interval::new(2, 3),
            Interval::new(2, 4),
            Interval::new(3, 5),
        ];

        let expected = 1;

        let got = find_minimum_meeting_rooms(&mut arr);
        assert_eq!(expected, got);
    }
}
