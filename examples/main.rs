use std::collections::BTreeMap as SortedMap;

#[ez::main]
async fn main(args: Vec<String>, _env: SortedMap<String, String>) {
    println!("{}", x(args.len() as _).await?);
}

#[ez::throws(std::num::ParseIntError)]
async fn x(n: i32) -> i32 {
    if n == 0 {
        throw!("nope".parse::<i32>().unwrap_err());
    } else {
        n * 2
    }
}
