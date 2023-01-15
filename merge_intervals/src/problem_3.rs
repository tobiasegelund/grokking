use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::Interval;

#[derive(Debug)]
struct EmployeeInterval {
    interval: Interval,
    employee_index: usize,
    interval_index: usize,
}

impl EmployeeInterval {
    fn new(interval: Interval, employee_index: usize, interval_index: usize) -> Self {
        Self {
            interval,
            employee_index,
            interval_index,
        }
    }
}

impl Ord for EmployeeInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.start.cmp(&other.interval.start)
    }
}

impl PartialOrd for EmployeeInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EmployeeInterval {
    fn eq(&self, other: &Self) -> bool {
        self.interval.start == other.interval.start
    }
}

impl Eq for EmployeeInterval {}

fn find_employee_free_time(schedule: &Vec<Vec<Interval>>) -> Vec<Interval> {
    if schedule.is_empty() {
        return vec![];
    }

    let n = schedule.len();
    let mut result = vec![];
    let mut min_heap = BinaryHeap::new();

    for i in 0..n {
        min_heap.push(EmployeeInterval {
            interval: schedule[i][0],
            employee_index: i,
            interval_index: 0,
        });
    }

    let mut previous_interval = min_heap.peek().unwrap().interval;
    while let Some(queue_top) = min_heap.pop() {
        if previous_interval.end < queue_top.interval.start {
            result.push(Interval {
                start: previous_interval.end,
                end: queue_top.interval.start,
            });
            previous_interval = queue_top.interval;
        } else {
            if previous_interval.end < queue_top.interval.end {
                previous_interval = queue_top.interval;
            }
        }

        let employee_schedule = &schedule[queue_top.employee_index];
        if employee_schedule.len() > queue_top.interval_index + 1 {
            min_heap.push(EmployeeInterval {
                interval: employee_schedule[queue_top.interval_index + 1],
                employee_index: queue_top.employee_index,
                interval_index: queue_top.interval_index + 1,
            });
        }
    }
    result
}
