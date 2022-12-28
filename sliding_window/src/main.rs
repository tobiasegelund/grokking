fn find_averages_of_subarrays_poor(k: i32, arr: Vec<i32>) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();
    // let _len: i32 = arr.len() + k - 1;

    arr.iter_mut().enumerate()

    result
}

fn main() {
    let result = find_averages_of_subarrays_poor(5, vec![1, 3, 2, 6, -1, 4, 1, 8, 2]);
    println!("{:?}", result);
    assert_eq!(result, vec![2.2, 2.8, 2.4, 3.6, 2.8]);
}
