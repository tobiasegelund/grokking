use crate::CPU;

fn maximum_cpu_load(arr: &mut Vec<CPU>) -> i32 {
    let mut maxmimum_load = 0;
    let mut sum = 0;
    arr.sort_by(|a, b| a.start.cmp(&b.start));
    let mut end = arr[0].end;

    for i in 0..arr.len() {
        let cpu = &arr[i];
        if cpu.start <= end {
            sum += cpu.load;
            maxmimum_load = std::cmp::max(sum, maxmimum_load);
            end = std::cmp::max(end, cpu.end);
        } else {
            sum = 0;
        }
    }

    maxmimum_load
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_maximum_cpu_load() {
        let mut arr = vec![CPU::new(7, 9, 6), CPU::new(1, 4, 3), CPU::new(2, 5, 4)];

        let expected = 7;

        let got = maximum_cpu_load(&mut arr);
        assert_eq!(expected, got);
    }
}
