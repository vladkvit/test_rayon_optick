use test_rayon::{multi_fib, single_fib};

fn bench() {
    optick::event!();

    let inputs: Vec<i32> = (1..=40).collect();
    let results = multi_fib(&inputs);
    println!("Fibonacci results: {:?}", results);

    let inputs: Vec<i32> = (1..=30).collect();
    let results = single_fib(&inputs);
    println!("Fibonacci results: {:?}", results);
}

fn main() {
    optick::start_capture();
    bench();
    optick::stop_capture("capture_name");
}
