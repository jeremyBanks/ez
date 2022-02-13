use ez::throw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match "2.5".parse::<i32>() {
        Err(e) => throw!(e),
        _ => throw!("hello, world"),
    }
}
