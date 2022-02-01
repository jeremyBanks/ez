use ez::*;

fn main() {
    let _a = alice();
    // let _a = try_alice()?;
    // let _f = try_alice();

    // let _b = bob("1");
    // let _b = try_bob("threeve")?;
    // let _b = try_bob("3");
}

/// This is Alice.
#[try_or_panics]
pub fn alice() -> i64 {
    ("s").parse()?
}

#[throws]
/// This is Bob.
pub fn bob(n: &str) -> i64 {
    n.parse()?
}

trait Foo {
    #[try_or_panics]
    fn foo(&self, n: i64) -> i64 {
        n
    }
}

struct Bar;

impl Foo for Bar {
    fn try_foo(&self, n: i64) -> Result<i64, eyre::Report> {
        try_alice()
    }
}
