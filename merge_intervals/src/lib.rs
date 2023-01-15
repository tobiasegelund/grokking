mod conflicting_appointments;
mod insert_interval;
mod intervals_intersection;
mod merge_intervals;
mod problem_1;
mod problem_2;
mod problem_3;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct CPU {
    start: i32,
    end: i32,
    load: i32,
}

impl CPU {
    fn new(start: i32, end: i32, load: i32) -> Self {
        Self { start, end, load }
    }
}
