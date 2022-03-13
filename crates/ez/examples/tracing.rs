use ez::prelude::*;

#[main]
fn main() {
    print_double(1, "twice one is");
}

#[instrument]
fn print_double(n: i32, message: &str) {
    let doubled = n * 2 + 1;
    info!("{message} {n}");
    double_check_result(doubled, n);
}

#[instrument]
fn double_check_result(result: i32, n: i32) {
    // compute the result using addition instead of multiplication
    let doubled = n + n;
    if doubled != result {
        error!("double of {n} is {doubled}, but our result was {result}!");
    }
}
