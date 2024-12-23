use test_rayon::multi_fib;

fn main() {
    optick::start_capture();
    let inputs: Vec<i32> = (1..=40).collect();
    let results = multi_fib(&inputs);
    println!("Fibonacci results: {:?}", results);
    optick::stop_capture("capture_name");
}
