use ez::{main, try_throws};

#[main]
async fn main(mut args: Vec<String>) -> i32 {
    args.push("hello".to_string());
    Struct(args).try_inherent_method(1)?
}

pub struct Struct(Vec<String>);

impl Struct {
    #[try_throws]
    pub fn inherent_method(&mut self, arg: i32) -> i32 {
        if arg >= 0 {
            self.0.push(arg.to_string());
            self.0.len() as i32
        } else {
            throw!("arg must be non-negative");
        }
    }
}
