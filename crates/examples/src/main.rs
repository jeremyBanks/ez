fn main() -> Result<(), ez::Error> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = std::process::Command::new(cargo);
    command.args(&["run", "--example"]).args(std::env::args().skip(1));
    eprintln!("{:?}", &command);

    command.status()?;

    Ok(())
}
