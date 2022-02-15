use ez::{main, throws, try_throws};

#[main]
async fn main() -> u8 {
    0
}

struct StructA;
struct StructB;
struct StructC;
struct StructD;

impl StructA {
    #[try_throws]
    fn inherent_method(&self, arg: i32) -> i32 {
        if arg >= 0 {
            arg
        } else {
            throw!("arg must be non-negative");
        }
    }
}

trait TraitA {
    #[try_throws]
    fn method(&self) -> i32;
}

trait TraitB: Sized {
    #[throws]
    fn fallible(self);

    #[try_throws]
    fn also_fallible(self);
}

impl TraitA for StructA {
    fn method(&self) -> i32 {
        self.inherent_method(1)
    }

    fn try_method(&self) -> Result<i32, ez::Error> {
        self.try_inherent_method(1)
    }
}

impl TraitA for StructB {
    fn method(&self) -> i32 {
        2
    }

    #[throws]
    fn try_method(&self) -> i32 {
        2
    }
}

impl TraitA for StructC {
    #[try_throws]
    fn method(&self) -> i32 {
        3
    }
}

impl TraitA for StructD {
    fn try_method(&self) -> Result<i32, ez::Error> {
        Ok(3)
    }
}
