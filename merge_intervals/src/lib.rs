mod conflicting_appointments;
mod insert_interval;
mod intervals_intersection;
mod merge_intervals;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}
