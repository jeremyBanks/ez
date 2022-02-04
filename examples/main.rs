use {std::collections::BTreeMap as SortedMap, tracing::debug};

#[ez::main]
fn main(args: Vec<String>, env: SortedMap<String, String>) -> eyre::Result<()> {
    debug!("args: {args:?}");
    debug!("env: {env:#?}");
}
