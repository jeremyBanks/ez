use {std::collections::BTreeMap as SortedMap};

#[ez::main]
fn main(args: Vec<String>, env: SortedMap<String, String>) -> eyre::Result<()> {
    x(args.len() as _);
}

#[ez::panics]
fn x(n: i32) -> i32 {
    if n % 2 == 0 {
        n
    } else {
        throw!("n must be even");
    }
}
