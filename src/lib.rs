use rayon::prelude::*;

pub fn fib(n: i32) -> i32 {
    optick::event!(); // This event is NOT captured
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub fn multi_fib(nums: &Vec<i32>) -> Vec<i32> {
    optick::event!(); // This event is captured

    // Parallel map each element in `nums` through `fib` and collect into a Vec<i32>
    nums.par_iter().map(|&num| fib(num)).collect()
}

pub fn single_fib(nums: &Vec<i32>) -> Vec<i32> {
    optick::event!(); // This event is captured

    // Parallel map each element in `nums` through `fib` and collect into a Vec<i32>
    nums.iter().map(|&num| fib(num)).collect()
}
