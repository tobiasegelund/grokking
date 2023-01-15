use crate::{merge_intervals, Interval};

fn find_intersection(arr1: &mut Vec<Interval>, arr2: &mut Vec<Interval>) -> Vec<Interval> {
    let mut merged_intervals: Vec<Interval> = Vec::new();

    for i in arr1.iter() {
        for j in arr2.iter() {
            if (i.start >= j.start) || (i.end <= j.end) {
                let start = std::cmp::max(i.start, j.start);
                let end = std::cmp::min(i.end, j.end);
                if start <= end {
                    merged_intervals.push(Interval::new(start, end));
                }
            }
        }
    }

    merged_intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_intersection() {
        let mut arr1 = vec![
            Interval::new(1, 3),
            Interval::new(5, 6),
            Interval::new(7, 9),
        ];

        let mut arr2 = vec![Interval::new(2, 3), Interval::new(5, 7)];

        let expected = vec![
            Interval::new(2, 3),
            Interval::new(5, 6),
            Interval::new(7, 7),
        ];
        let got = find_intersection(&mut arr1, &mut arr2);

        assert_eq!(expected, got);
    }
}
