use crate::Interval;

fn insert_interval(arr: &mut Vec<Interval>, new_interval: Interval) -> Vec<Interval> {
    let mut start = new_interval.start;
    let mut end = new_interval.end;

    let mut merged_intervals: Vec<Interval> = Vec::new();

    for i in 0..arr.len() {
        let interval = &arr[i];
        if interval.end > new_interval.start {
            if interval.start <= new_interval.end {
                start = std::cmp::min(start, interval.start);
                end = std::cmp::max(end, interval.end);

                merged_intervals.push(Interval::new(start, end));
            } else {
                merged_intervals.push(interval.clone())
            }
        } else {
            merged_intervals.push(interval.clone())
        }
    }
    merged_intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_interval() {
        let mut arr = vec![
            Interval::new(1, 3),
            Interval::new(5, 7),
            Interval::new(8, 12),
        ];

        let expected = vec![
            Interval::new(1, 3),
            Interval::new(4, 7),
            Interval::new(8, 12),
        ];
        let got = insert_interval(&mut arr, Interval::new(4, 6));

        assert_eq!(expected, got);
    }
}
