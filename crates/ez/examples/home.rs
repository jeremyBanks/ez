use {ez::main, std::collections::HashMap};

#[main]
fn main(_args: Vec<String>, env: HashMap<String, String>) -> u8 {
    let home = env.get("HOME");
    match home {
        Some(path) => {
            println!("HOME is: {path}");
            return 0;
        },
        None => {
            println!("fatal error: HOME not set");
            return 1;
        },
    }
}
