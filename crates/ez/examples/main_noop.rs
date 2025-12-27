#[ez::main]
#[allow(clippy::let_unit_value)] // false positive: macro transforms this into Result-returning fn
fn main() {
    println!("hello, world!");
}
