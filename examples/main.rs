use std::collections::BTreeMap;

#[ez::main]
fn main(args: Vec<String>, env: BTreeMap<String, String>) -> eyre::Result<()> {
    tracing::debug!("args: {args:?}");
    log::debug!("env: {env:#?}");
}
