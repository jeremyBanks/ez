use ez::prelude::*;

#[ez::main]
fn main(args: Vec<String>, env: OrderedMap<String, String>) -> eyre::Result<()> {
    debug!("args: {args:?}");
    debug!("env: {env:#?}");
}
