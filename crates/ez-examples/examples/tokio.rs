use ez::__::tokio;

#[ez::main]
async fn main() {
    let contents = tokio::fs::read("Cargo.toml").await?;
    if contents.is_empty() {
        throw!("the file was empty");
    }
    println!("{contents:?}");
}
