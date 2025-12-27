#[ez::main]
#[allow(clippy::let_unit_value)] // false positive: macro transforms this into Result-returning fn
async fn main() {
    let contents = tokio::fs::read("Cargo.toml").await?;
    if contents.is_empty() {
        throw!("the file was empty");
    }
    println!("{contents:?}");
}
