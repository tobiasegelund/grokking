use crate::Interval;

fn merge_overlapping_intervals(arr: &mut Vec<Interval>) -> Vec<Interval> {
    let mut merged_intervals: Vec<Interval> = Vec::new();
    arr.sort_by(|a, b| a.start.cmp(&b.start));

    let mut start = arr[0].start;
    let mut end = arr[0].end;

    for i in 1..arr.len() {
        let interval = &arr[i];
        if interval.start <= end {
            end = std::cmp::max(interval.end, end);
        } else {
            merged_intervals.push(Interval::new(start, end));
            start = interval.start;
            end = interval.end;
        }
    }
    merged_intervals.push(Interval::new(start, end));

    merged_intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_overlapping_intervals() {
        let mut arr = vec![
            Interval::new(1, 4),
            Interval::new(2, 5),
            Interval::new(7, 9),
        ];

        let expected = vec![Interval::new(1, 5), Interval::new(7, 9)];

        let got = merge_overlapping_intervals(&mut arr);
        assert_eq!(expected, got);
    }
}
