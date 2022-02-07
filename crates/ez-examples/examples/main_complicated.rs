use std::collections::HashMap;

#[ez::main]
fn main(args: Vec<String>, env: HashMap<String, String>) -> u8 {
    println!(
        "Are you at HOME? {}",
        env.get("HOME").map(String::from).unwrap_or_default()
    );
    println!(
        "            PWD  {}",
        env.get("PWD").map(String::from).unwrap_or_default()
    );
    println!("           args  {:?}", args);

    return Ok(8);
}
