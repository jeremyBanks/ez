use std::collections::HashMap;

#[ez::main]
async fn main(args: Vec<String>, env: HashMap<String, String>) -> u8 {
    println!(
        "Are you at HOME? {}",
        env.get("HOME").map(String::from).unwrap_or_default()
    );
    println!(
        "            PWD  {}",
        env.get("PWD").map(String::from).unwrap_or_default()
    );
    println!("           args  {:?}", args);

    Ok("8".parse()?)
}
